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

#include "FixRustAllocLTOPass.hpp"

#include "../../Error.hpp"
#include <llvm/IR/Constants.h>
#include <llvm/IR/Function.h>
#include <llvm/IR/GlobalVariable.h>
#include <llvm/IR/Module.h>
#include <llvm/Support/Regex.h>

#include <string>
#include <vector>

using namespace llvm;

auto FixRustAllocLTOPass::run(Module &M, ModuleAnalysisManager &MAM) -> PreservedAnalyses
{
	auto modified = false;

	/* Fat LTO can rename __rust_alloc to __rust_alloc.NN (internal linkage).
	 * GenMC intercepts allocations by exact name match, so we need to
	 * restore the canonical names. */
	static const char *allocFuns[] = {
		"__rust_alloc",
		"__rust_dealloc",
		"__rust_realloc",
		"__rust_alloc_zeroed",
	};

	for (const char *baseName : allocFuns) {
		std::string pattern = std::string("^") + baseName + "\\.[0-9]+$";
		llvm::Regex re(pattern);

		std::vector<Function *> toFix;
		for (auto &F : M) {
			if (re.match(F.getName()))
				toFix.push_back(&F);
		}

		/* Get or create the canonical external declaration.
		 * GenMC intercepts these functions by name, so they must be
		 * bodyless declarations for interception to work. */
		Function *canonical = M.getFunction(baseName);
		if (!canonical && !toFix.empty()) {
			canonical = Function::Create(
				toFix[0]->getFunctionType(),
				GlobalValue::ExternalLinkage,
				baseName, &M);
		}

		/* Replace all .NN suffixed variants with the canonical declaration */
		for (Function *F : toFix) {
			F->replaceAllUsesWith(canonical);
			F->eraseFromParent();
			modified = true;
		}

		/* Strip the body from the canonical function itself.
		 * Fat LTO may also produce unsuffixed versions with bodies
		 * (e.g. __rust_dealloc, __rust_realloc with internal linkage). */
		if (canonical && !canonical->isDeclaration()) {
			canonical->deleteBody();
			canonical->setLinkage(GlobalValue::ExternalLinkage);
			modified = true;
		}
	}

	/* Defensively delete bodies of __rdl_alloc, __rdl_dealloc, etc.
	 * These are the default Rust allocator implementations that call
	 * posix_memalign/free. With GenMC intercepting __rust_alloc,
	 * these bodies are dead code and can cause issues if reached. */
	static const char *rdlFuns[] = {
		"__rdl_alloc",
		"__rdl_dealloc",
		"__rdl_realloc",
		"__rdl_alloc_zeroed",
	};

	for (const char *name : rdlFuns) {
		if (Function *F = M.getFunction(name)) {
			if (!F->isDeclaration()) {
				F->deleteBody();
				modified = true;
			}
		}
	}

	/* Pre-initialize the thread stack size sentinel to skip env var lookup.
	 * The standard library's thread::Builder::spawn reads RUST_MIN_STACK
	 * via SmallCString/memchr, which GenMC cannot handle. Setting MIN to
	 * DEFAULT_MIN_STACK_SIZE + 1 (the sentinel format) makes the code
	 * return the default immediately without touching env vars. */
	{
		llvm::Regex minRe("^_ZN3std6thread7Builder16spawn_unchecked_.*3MIN");
		for (auto &GV : M.globals()) {
			if (!minRe.match(GV.getName()))
				continue;
			/* DEFAULT_MIN_STACK_SIZE on Linux = 2 MiB; sentinel = amt + 1 */
			constexpr uint64_t sentinel = 2 * 1024 * 1024 + 1;
			auto *i64Ty = Type::getInt64Ty(M.getContext());
			auto *initVal = ConstantInt::get(i64Ty, sentinel);

			/* The global is typed as <{ [8 x i8] }> (packed struct from
			 * AtomicUsize). Create a new i64 global, RAUW, and erase the
			 * old one so the interpreter handles loads as scalar i64. */
			auto *newGV = new GlobalVariable(
				M, i64Ty, false, GV.getLinkage(), initVal,
				GV.getName() + ".fixed");
			newGV->setAlignment(GV.getAlign());
			GV.replaceAllUsesWith(newGV);
			newGV->takeName(&GV);
			GV.eraseFromParent();
			modified = true;
			break; /* iterator invalidated */
		}
	}

	/* Pre-initialize the DLSYM global for __pthread_get_minstack to null.
	 * The standard library uses dlsym(NULL, "__pthread_get_minstack") to
	 * look up the glibc-internal function for minimum stack size.  The
	 * returned pointer is native code that GenMC cannot interpret.
	 * Setting the DLSYM global to null (0) makes the code skip the
	 * dlsym call and use the default minimum stack size (16384). */
	{
		llvm::Regex dlsymRe("^_ZN3std3sys3pal4unix6thread14min_stack_size5DLSYM");
		for (auto &GV : M.globals()) {
			if (!dlsymRe.match(GV.getName()))
				continue;
			auto *ptrTy = GV.getValueType();
			auto *nullVal = ConstantPointerNull::get(
				cast<PointerType>(ptrTy));
			GV.setInitializer(nullVal);
			modified = true;
			break;
		}
	}

	/* Make register_dtor a no-op. The standard library registers TLS
	 * destructors via __cxa_thread_atexit_impl, but GenMC's atexit
	 * override doesn't pass the data argument, causing an arg count
	 * mismatch.  TLS cleanup is not needed for model checking. */
	{
		llvm::Regex dtorRe("^_ZN3std3sys3pal4unix17thread_local_dtor13register_dtor");
		for (auto &F : M) {
			if (!dtorRe.match(F.getName()) || F.isDeclaration())
				continue;
			F.deleteBody();
			auto *BB = BasicBlock::Create(M.getContext(), "entry", &F);
			ReturnInst::Create(M.getContext(), BB);
			modified = true;
		}
	}

	return modified ? PreservedAnalyses::none() : PreservedAnalyses::all();
}
