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
 */

#include "BypassRustRuntimePass.hpp"

#include <llvm/IR/Constants.h>
#include <llvm/IR/Function.h>
#include <llvm/IR/IRBuilder.h>
#include <llvm/IR/Instructions.h>
#include <llvm/IR/Module.h>
#include <llvm/Support/Regex.h>

using namespace llvm;

auto BypassRustRuntimePass::run(Module &M, ModuleAnalysisManager &MAM) -> PreservedAnalyses
{
	auto *mainFn = M.getFunction("main");
	if (!mainFn || mainFn->isDeclaration())
		return PreservedAnalyses::all();

	/* Look for a call to lang_start in @main. The pattern is:
	 *   call lang_start(ptr @user_main, i64 %argc, ptr %argv, i8 %sigpipe)
	 * where the first argument is the user's actual main function. */
	llvm::Regex langStartRe("^_ZN3std2rt10lang_start");
	Function *userMain = nullptr;

	for (auto &BB : *mainFn) {
		for (auto &I : BB) {
			auto *call = dyn_cast<CallInst>(&I);
			if (!call)
				continue;
			auto *callee = call->getCalledFunction();
			if (!callee || !langStartRe.match(callee->getName()))
				continue;
			if (call->arg_size() < 1)
				continue;
			userMain = dyn_cast<Function>(call->getArgOperand(0));
			break;
		}
		if (userMain)
			break;
	}

	if (!userMain)
		return PreservedAnalyses::all();

	/* Rewrite @main to call the user's main directly and return 0. */
	mainFn->deleteBody();
	auto *BB = BasicBlock::Create(M.getContext(), "entry", mainFn);
	IRBuilder<> builder(BB);
	builder.CreateCall(userMain);
	builder.CreateRet(ConstantInt::get(Type::getInt32Ty(M.getContext()), 0));

	return PreservedAnalyses::none();
}
