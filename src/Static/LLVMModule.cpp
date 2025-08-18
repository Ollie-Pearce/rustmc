/*
 * GenMC -- Generic Model Checking.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, you can access it online at
 * http://www.gnu.org/licenses/gpl-3.0.html.
 *
 * Author: Michalis Kokologiannakis <michalis@mpi-sws.org>
 */

#include "LLVMModule.hpp"
#include "Transforms/BisimilarityCheckerPass.hpp"
#include "Transforms/CallInfoCollectionPass.hpp"
#include "Transforms/CodeCondenserPass.hpp"
#include "Transforms/ConfirmationAnnotationPass.hpp"
#include "Transforms/DeclareInternalsPass.hpp"
#include "Transforms/DefineLibcFunsPass.hpp"
#include "Transforms/EliminateAnnotationsPass.hpp"
#include "Transforms/EliminateCASPHIsPass.hpp"
#include "Transforms/EliminateCastsPass.hpp"
#include "Transforms/EliminateRedundantInstPass.hpp"
#include "Transforms/EliminateUnusedCodePass.hpp"
#include "Transforms/EscapeCheckerPass.hpp"
#include "Transforms/FunctionInlinerPass.hpp"
#include "Transforms/IntrinsicLoweringPass.hpp"
#include "Transforms/LoadAnnotationPass.hpp"
#include "Transforms/LocalSimplifyCFGPass.hpp"
#include "Transforms/LoopJumpThreadingPass.hpp"
#include "Transforms/LoopUnrollPass.hpp"
#include "Transforms/MDataCollectionPass.hpp"
#include "Transforms/MMDetectorPass.hpp"
#include "Transforms/PromoteMemIntrinsicPass.hpp"
#include "Transforms/PropagateAssumesPass.hpp"
#include "Transforms/PromoteMemcpy.hpp"
#include "Transforms/InsertUndefs.hpp"
#include "Transforms/PromoteMemMove.hpp"
#include "Transforms/PromotePause.hpp"
#include "Transforms/splitRW.hpp"
#include "Transforms/SpinAssumePass.hpp"
#include "Transforms/CollectIntrinsics.hpp"
#include "../Error.hpp"
#include "../SExprVisitor.hpp"

#include <llvm/Analysis/AliasAnalysis.h>
#include <llvm/Analysis/BasicAliasAnalysis.h>
#include <llvm/Analysis/GlobalsModRef.h>
#include <llvm/Analysis/TypeBasedAliasAnalysis.h>
#include <llvm/Bitcode/BitcodeReader.h>
#include <llvm/Bitcode/BitcodeWriter.h>
#include <llvm/IR/IRPrintingPasses.h>
#include <llvm/IR/LLVMContext.h>
#include <llvm/IR/PassManager.h>
#include <llvm/IR/Verifier.h>
#include <llvm/IRReader/IRReader.h>
#include <llvm/Passes/PassBuilder.h>
#include <llvm/Passes/PassPlugin.h>
#include <llvm/Support/Debug.h>
#include <llvm/Support/FileSystem.h>
#include <llvm/Support/MemoryBuffer.h>
#include <llvm/Support/raw_ostream.h>
#include <llvm/Transforms/IPO/DeadArgumentElimination.h>
#include <llvm/Transforms/InstCombine/InstCombine.h>
#include <llvm/Transforms/Scalar/JumpThreading.h>
#include <llvm/Transforms/Utils/Mem2Reg.h>

namespace LLVMModule {

auto parseLLVMModule(const std::string &filename,
		     const std::unique_ptr<llvm::LLVMContext> &ctx) -> std::unique_ptr<llvm::Module>
{
	llvm::SMDiagnostic err;

	auto mod = llvm::parseIRFile(filename, err, *ctx);
	if (!mod) {
		err.print(filename.c_str(), llvm::dbgs());
		ERROR("Could not parse LLVM IR!\n");
	}
	return std::move(mod);
}

auto cloneModule(const std::unique_ptr<llvm::Module> &mod,
		 const std::unique_ptr<llvm::LLVMContext> &ctx) -> std::unique_ptr<llvm::Module>
{
	/* Roundtrip the module to a stream and then back into the new context */
	std::string str;
	llvm::raw_string_ostream stream(str);

	llvm::WriteBitcodeToFile(*mod, stream);

	llvm::StringRef ref(stream.str());
	std::unique_ptr<llvm::MemoryBuffer> buf(llvm::MemoryBuffer::getMemBuffer(ref));

	return std::move(llvm::parseBitcodeFile(buf->getMemBufferRef(), *ctx).get());
}

void initializeVariableInfo(ModuleInfo &MI, PassModuleInfo &PI)
{
	for (auto &kv : PI.varInfo.globalInfo)
		MI.varInfo.globalInfo[MI.idInfo.VID.at(kv.first)] = kv.second;
	for (auto &kv : PI.varInfo.localInfo) {
		if (MI.idInfo.VID.count(kv.first))
			MI.varInfo.localInfo[MI.idInfo.VID.at(kv.first)] = kv.second;
	}
	MI.varInfo.internalInfo = PI.varInfo.internalInfo;
}

void initializeAnnotationInfo(ModuleInfo &MI, PassModuleInfo &PI)
{
	using Transformer = SExprTransformer<llvm::Value *>;
	Transformer tr;

	for (auto &kv : PI.annotInfo.annotMap) {
		MI.annotInfo.annotMap[MI.idInfo.VID.at(kv.first)] = tr.transform(
			&*kv.second, [&](llvm::Value *v) { return MI.idInfo.VID.at(v); });
	}
}

void initializeFsInfo(ModuleInfo &MI, PassModuleInfo &PI)
{
	MI.fsInfo.filenames.insert(PI.filenames.begin(), PI.filenames.end());
}

void initializeModuleInfo(ModuleInfo &MI, PassModuleInfo &PI)
{
	MI.collectIDs();
	initializeVariableInfo(MI, PI);
	initializeAnnotationInfo(MI, PI);
	initializeFsInfo(MI, PI);
	MI.determinedMM = PI.determinedMM;
}

auto transformLLVMModule(llvm::Module &mod, ModuleInfo &MI,
			 const std::shared_ptr<const Config> &conf) -> bool
{
	PassModuleInfo PI;

	/* NOTE: The order between the analyses, the builder and the managers matters */

	/* First, register the analyses that we are about to use.
	 * We also use an (unused) pass builder to load default analyses */
	llvm::LoopAnalysisManager lam;
	llvm::CGSCCAnalysisManager cgam;
	llvm::FunctionAnalysisManager fam;
	llvm::ModuleAnalysisManager mam;
	//llvm::ModulePassManager mpm;

	mam.registerPass([&] { return MDataInfo(); });
	mam.registerPass([&] { return MMAnalysis(); });
	mam.registerPass([&] { return CallAnalysis(); });
	mam.registerPass([&] { return EscapeAnalysis(); });
	//mam.registerPass([&] { return MyModuleAnalysis(); });
	fam.registerPass([&] { return BisimilarityAnalysis(); });
	fam.registerPass([&] { return LoadAnnotationAnalysis(); });


	fam.registerPass([&] { return AAManager(); });
	fam.registerPass([&] { return BasicAA(); });
	fam.registerPass([&] { return TypeBasedAA(); });

	//mpm.addPass(EvalAAPass());

	llvm::PassBuilder pb;
	pb.registerModuleAnalyses(mam);
	pb.registerCGSCCAnalyses(cgam);
	pb.registerFunctionAnalyses(fam);
	pb.registerLoopAnalyses(lam);
	pb.crossRegisterProxies(lam, fam, cgam, mam);

	//mpm.run(mod, mam);


	/* Then create two pass managers: a basic one and one that
	runs some loop passes */
	llvm::ModulePassManager basicOptsMGR;

	basicOptsMGR.addPass(DeclareInternalsPass());
	basicOptsMGR.addPass(DefineLibcFunsPass());
	basicOptsMGR.addPass(MDataCollectionPass(PI));
	
	

	if (conf->inlineFunctions)
		basicOptsMGR.addPass(FunctionInlinerPass());
	{
		llvm::FunctionPassManager fpm;
		/* Run after the inliner because it might generate new memcpys */
		//
		fpm.addPass(PromoteMemcpy());


		
		fpm.addPass(PromoteMemIntrinsicPass());
		
		fpm.addPass(PromoteMemMove());
		
		fpm.addPass(PromotePause());
		fpm.addPass(CollectIntrinsics());
		
		fpm.addPass(IntrinsicLoweringPass());
		

		if (conf->castElimination)
			fpm.addPass(EliminateCastsPass());
		fpm.addPass(PromotePass());
		basicOptsMGR.addPass(llvm::createModuleToFunctionPassAdaptor(std::move(fpm)));
	}
	
	basicOptsMGR.addPass(DeadArgumentEliminationPass());
	{
		llvm::FunctionPassManager fpm;
		fpm.addPass(LocalSimplifyCFGPass());
		fpm.addPass(EliminateAnnotationsPass());
		fpm.addPass(EliminateRedundantInstPass());
		basicOptsMGR.addPass(llvm::createModuleToFunctionPassAdaptor(std::move(fpm)));
	}

	if (conf->mmDetector)
		basicOptsMGR.addPass(MMDetectorPass(PI));

	auto preserved = basicOptsMGR.run(mod, mam);

	llvm::ModulePassManager loopOptsMGR;

	{
		llvm::FunctionPassManager fpm;
		fpm.addPass(EliminateCASPHIsPass());
		fpm.addPass(llvm::JumpThreadingPass());
		fpm.addPass(EliminateUnusedCodePass());
		if (conf->codeCondenser && !conf->checkLiveness)
			fpm.addPass(CodeCondenserPass());
		if (conf->loopJumpThreading)
			fpm.addPass(createFunctionToLoopPassAdaptor(LoopJumpThreadingPass()));

		
		loopOptsMGR.addPass(llvm::createModuleToFunctionPassAdaptor(std::move(fpm)));
	}
	if (conf->spinAssume)
		loopOptsMGR.addPass(SpinAssumePass(conf->checkLiveness));
	if (conf->unroll.has_value())
		loopOptsMGR.addPass(
			createModuleToFunctionPassAdaptor(createFunctionToLoopPassAdaptor(
				LoopUnrollPass(*conf->unroll, conf->noUnrollFuns))));
	preserved.intersect(loopOptsMGR.run(mod, mam));

	/* Run annotation passes last so that the module is stable */
	{
		llvm::FunctionPassManager fpm;
		if (conf->assumePropagation)
			fpm.addPass(PropagateAssumesPass());
		if (conf->confirmAnnot)
			fpm.addPass(ConfirmationAnnotationPass());
		if (conf->loadAnnot)
			fpm.addPass(LoadAnnotationPass(PI.annotInfo));

		
		fpm.addPass(InsertUndefs());
		basicOptsMGR.addPass(llvm::createModuleToFunctionPassAdaptor(std::move(fpm)));
	}
	preserved.intersect(basicOptsMGR.run(mod, mam));

	initializeModuleInfo(MI, PI);

	//assert(!llvm::verifyModule(mod, &llvm::dbgs()));
	return true;
}

void printLLVMModule(llvm::Module &mod, const std::string &out)
{
	auto flags =
#if LLVM_VERSION_MAJOR < 13
		llvm::sys::fs::F_None;
#else
		llvm::sys::fs::OF_None;
#endif
	std::error_code errs;
	auto os = std::make_unique<llvm::raw_fd_ostream>(out.c_str(), errs, flags);

	/* TODO: Do we need an exception? If yes, properly handle it */
	if (errs) {
		WARN("Failed to write transformed module to file " + out + ": " + errs.message());
		return;
	}
	mod.print(*os, nullptr);
}

} // namespace LLVMModule
