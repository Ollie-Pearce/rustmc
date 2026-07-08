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
#include <llvm/IR/IRBuilder.h>
#include <llvm/IR/Module.h>
#include <llvm/Support/Alignment.h>
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

	/* Recent rustc emits the allocator shim under the synthetic `__rustc`
	 * crate with v0 symbol mangling, e.g.
	 *   _RNvCsfgh5XYAs1hF_7___rustc12___rust_alloc  ("__rustc::__rust_alloc")
	 * These carry a per-build disambiguator hash (so an exact-name map can
	 * never match) AND they have bodies (they call __rdl_alloc ->
	 * posix_memalign). Since GenMC intercepts allocations by exact name on
	 * bodyless declarations, std-internal allocations (Arc::new_uninit in
	 * Thread::new, Box::new, ...) would otherwise run a real libc allocation
	 * that the model checker never registers, and the first store through
	 * the result trips "access to non-allocated memory". Canonicalise the
	 * v0 form to the bodyless legacy declaration, exactly as above.
	 *
	 * __rust_realloc is canonicalised here too. The interpreter has no
	 * native realloc model, but rather than leave the v0 body intact (it
	 * calls __rdl_realloc, whose body the block further down deletes ->
	 * "unknown external function" abort), we collapse every realloc form
	 * to one canonical declaration and synthesize a faithful body for it
	 * below (alloc + memcpy + dealloc), reusing the modelled primitives. */
	static const struct {
		const char *canonical;
		const char *v0Suffix;
	} v0AllocFuns[] = {
		{"__rust_alloc", "___rust_alloc"},
		{"__rust_dealloc", "___rust_dealloc"},
		{"__rust_alloc_zeroed", "___rust_alloc_zeroed"},
		{"__rust_realloc", "___rust_realloc"},
	};

	for (const auto &af : v0AllocFuns) {
		/* v0 mangled, ends in the (extra-underscore-separated) shim name.
		 * The `$` anchor keeps `___rust_alloc` from matching
		 * `___rust_alloc_zeroed` / `___rust_dealloc`. */
		llvm::Regex re(std::string("^_RN.*") + af.v0Suffix + "$");

		std::vector<Function *> toFix;
		for (auto &F : M) {
			if (re.match(F.getName()))
				toFix.push_back(&F);
		}
		if (toFix.empty())
			continue;

		Function *canonical = M.getFunction(af.canonical);
		if (!canonical) {
			canonical = Function::Create(
				toFix[0]->getFunctionType(),
				GlobalValue::ExternalLinkage,
				af.canonical, &M);
		}

		for (Function *F : toFix) {
			/* These shims share the Rust allocator ABI, so the
			 * types match; skip defensively if they ever don't. */
			if (F->getFunctionType() != canonical->getFunctionType())
				continue;
			F->replaceAllUsesWith(canonical);
			F->eraseFromParent();
			modified = true;
		}

		if (canonical && !canonical->isDeclaration()) {
			canonical->deleteBody();
			canonical->setLinkage(GlobalValue::ExternalLinkage);
			modified = true;
		}
	}

	/* Canonicalise the default Rust allocator implementation symbols
	 * (__rdl_*) to the public allocator hooks (__rust_*). Most code reaches
	 * these through __rust_* shims, but some optimized/LTO modules can call
	 * __rdl_* directly. Leaving a bodyless __rdl_realloc declaration behind
	 * causes an "unknown external function" abort in the interpreter. */
	static const struct {
		const char *canonical;
		const char *legacy;
		const char *v0Suffix;
	} rdlFuns[] = {
		{"__rust_alloc", "__rdl_alloc", "___rdl_alloc"},
		{"__rust_dealloc", "__rdl_dealloc", "___rdl_dealloc"},
		{"__rust_realloc", "__rdl_realloc", "___rdl_realloc"},
		{"__rust_alloc_zeroed", "__rdl_alloc_zeroed", "___rdl_alloc_zeroed"},
	};

	for (const auto &rf : rdlFuns) {
		std::vector<Function *> toFix;
		if (Function *F = M.getFunction(rf.legacy))
			toFix.push_back(F);

		llvm::Regex re(std::string("^_RN.*") + rf.v0Suffix + "$");
		for (auto &F : M) {
			if (re.match(F.getName()))
				toFix.push_back(&F);
		}

		for (Function *F : toFix) {
			Function *canonical = M.getFunction(rf.canonical);
			if (!canonical) {
				canonical = Function::Create(
					F->getFunctionType(),
					GlobalValue::ExternalLinkage,
					rf.canonical, &M);
			}
			if (F->getFunctionType() != canonical->getFunctionType())
				continue;
			F->replaceAllUsesWith(canonical);
			F->eraseFromParent();
			modified = true;
		}
	}

	/* Synthesize a body for the canonical __rust_realloc.
	 *
	 * The interpreter intercepts __rust_alloc (FN_Malloc), __rust_dealloc
	 * (FN_Free) and memcpy (FN_Memcpy), but has no native realloc model. By
	 * this point every realloc form (legacy, fat-LTO `.NN`, v0-mangled
	 * `__rustc::__rust_realloc`) has been collapsed onto a single bodyless
	 * `__rust_realloc` declaration and the __rdl_* implementations it used
	 * to call have had their bodies removed. Left as-is, any reallocating
	 * code (Vec/String/HashMap growth) would call a bodyless external and
	 * abort with "Tried to execute an unknown external function".
	 *
	 * Express realloc in terms of the modelled primitives, matching the
	 * Rust GlobalAlloc::realloc contract
	 * (__rust_realloc(ptr, old_size, align, new_size) -> *mut u8):
	 *
	 *   new = __rust_alloc(new_size, align)
	 *   if new != null:
	 *       memcpy(new, ptr, min(old_size, new_size))
	 *       __rust_dealloc(ptr, old_size, align)
	 *   return new
	 *
	 * On allocation failure (new == null) the old block is left untouched
	 * and null is returned, exactly as the real allocator behaves. The
	 * memcpy uses a dynamic (non-constant) length, so the later
	 * PromoteMemIntrinsicPass leaves it alone and IntrinsicLoweringPass
	 * lowers it to a @memcpy call that the interpreter tracks byte-wise.
	 *
	 * This only changes behaviour for modules that reach __rust_realloc —
	 * previously a hard abort — so it is regression-safe, consistent with
	 * the alloc/dealloc canonicalisation above. */
	if (Function *reallocF = M.getFunction("__rust_realloc")) {
		Function *allocF = M.getFunction("__rust_alloc");
		Function *deallocF = M.getFunction("__rust_dealloc");
		if (reallocF->isDeclaration() && allocF && deallocF &&
		    reallocF->arg_size() == 4 && allocF->arg_size() == 2 &&
		    deallocF->arg_size() == 3 &&
		    reallocF->getReturnType()->isPointerTy()) {
			auto &Ctx = M.getContext();
			auto *entryBB = BasicBlock::Create(Ctx, "entry", reallocF);
			auto *copyBB = BasicBlock::Create(Ctx, "copy", reallocF);
			auto *retBB = BasicBlock::Create(Ctx, "ret", reallocF);

			auto *ptr = reallocF->getArg(0);
			auto *oldSize = reallocF->getArg(1);
			auto *align = reallocF->getArg(2);
			auto *newSize = reallocF->getArg(3);

			IRBuilder<> B(entryBB);
			Value *newPtr =
				B.CreateCall(allocF, {newSize, align}, "rmc.realloc.new");
			Value *isNull = B.CreateICmpEQ(
				newPtr, ConstantPointerNull::get(
						cast<PointerType>(newPtr->getType())));
			B.CreateCondBr(isNull, retBB, copyBB);

			B.SetInsertPoint(copyBB);
			Value *lt = B.CreateICmpULT(oldSize, newSize);
			Value *minLen =
				B.CreateSelect(lt, oldSize, newSize, "rmc.realloc.min");
			B.CreateMemCpy(newPtr, MaybeAlign(1), ptr, MaybeAlign(1), minLen);
			B.CreateCall(deallocF, {ptr, oldSize, align});
			B.CreateBr(retBB);

			B.SetInsertPoint(retBB);
			B.CreateRet(newPtr);

			modified = true;
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

	/* Make the thread-local destructor guard a no-op. On glibc,
	 * std::sys::thread_local::guard::key::enable installs a thread-exit
	 * TLS-destructor callback via pthread_key_create/pthread_setspecific
	 * (see destructors/list.rs). GenMC does not override the POSIX TLS-key
	 * API, so those calls would FFI into real glibc with GenMC-abstract
	 * pointers and segfault. As with register_dtor above, thread-exit TLS
	 * cleanup is not needed for model checking, so the guard is elided
	 * (returns void). This must run before linking so the now-unreferenced
	 * pthread_key_create/setspecific/key_delete declarations stay dead. */
	{
		llvm::Regex guardRe("^_ZN3std3sys12thread_local5guard3key6enable");
		for (auto &F : M) {
			if (!guardRe.match(F.getName()) || F.isDeclaration())
				continue;
			F.deleteBody();
			auto *BB = BasicBlock::Create(M.getContext(), "entry", &F);
			ReturnInst::Create(M.getContext(), BB);
			modified = true;
		}
	}

	return modified ? PreservedAnalyses::none() : PreservedAnalyses::all();
}
