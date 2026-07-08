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

#include "RewriteFutexSyscallPass.hpp"

#include <llvm/IR/Constants.h>
#include <llvm/IR/Function.h>
#include <llvm/IR/IRBuilder.h>
#include <llvm/IR/Instructions.h>
#include <llvm/IR/Module.h>

#include <vector>

using namespace llvm;

static constexpr int64_t NR_FUTEX = 202;

static constexpr int32_t FUTEX_OP_MASK_ = 0x7F;
static constexpr int32_t FUTEX_WAIT_ = 0;
static constexpr int32_t FUTEX_WAKE_ = 1;
static constexpr int32_t FUTEX_WAIT_BITSET_ = 9;

static Function *getOrCreateAssume(Module &M)
{
	auto *fnTy = FunctionType::get(Type::getVoidTy(M.getContext()),
				       {Type::getInt1Ty(M.getContext())}, false);
	return cast<Function>(
		M.getOrInsertFunction("__VERIFIER_assume", fnTy).getCallee());
}

static void rewriteFutexWait(CallInst *CI, Function *assumeFn)
{
	/* syscall(SYS_futex, addr, op, expected_val, ...) */
	auto *addr = CI->getArgOperand(1);
	auto *expectedVal = CI->getArgOperand(3);

	IRBuilder<> builder(CI);
	auto &ctx = CI->getContext();

	auto *loaded = builder.CreateLoad(Type::getInt32Ty(ctx), addr, true);
	cast<LoadInst>(loaded)->setOrdering(AtomicOrdering::Acquire);
	cast<LoadInst>(loaded)->setAlignment(Align(4));

	auto *cmp = builder.CreateICmpNE(loaded, expectedVal);
	builder.CreateCall(assumeFn, {cmp});

	CI->replaceAllUsesWith(ConstantInt::get(Type::getInt64Ty(ctx), 0));
	CI->eraseFromParent();
}

static void rewriteFutexWake(CallInst *CI)
{
	IRBuilder<> builder(CI);
	CI->replaceAllUsesWith(ConstantInt::get(Type::getInt64Ty(CI->getContext()), 1));
	CI->eraseFromParent();
}

static void rewriteNonFutexSyscall(CallInst *CI)
{
	IRBuilder<> builder(CI);
	CI->replaceAllUsesWith(ConstantInt::get(Type::getInt64Ty(CI->getContext()), 0));
	CI->eraseFromParent();
}

auto RewriteFutexSyscallPass::run(Module &M, ModuleAnalysisManager &MAM) -> PreservedAnalyses
{
	auto *syscallFn = M.getFunction("syscall");
	if (!syscallFn)
		return PreservedAnalyses::all();

	Function *assumeFn = getOrCreateAssume(M);

	std::vector<CallInst *> toRewrite;
	for (auto *user : syscallFn->users()) {
		if (auto *CI = dyn_cast<CallInst>(user))
			toRewrite.push_back(CI);
	}

	if (toRewrite.empty())
		return PreservedAnalyses::all();

	for (auto *CI : toRewrite) {
		auto *sysnoConst = dyn_cast<ConstantInt>(CI->getArgOperand(0));
		if (!sysnoConst) {
			rewriteNonFutexSyscall(CI);
			continue;
		}
		auto sysno = sysnoConst->getSExtValue();

		if (sysno != NR_FUTEX) {
			rewriteNonFutexSyscall(CI);
			continue;
		}

		auto *opConst = dyn_cast<ConstantInt>(CI->getArgOperand(2));
		if (!opConst) {
			rewriteNonFutexSyscall(CI);
			continue;
		}
		auto op = opConst->getZExtValue() & FUTEX_OP_MASK_;

		if (op == FUTEX_WAIT_ || op == FUTEX_WAIT_BITSET_)
			rewriteFutexWait(CI, assumeFn);
		else if (op == FUTEX_WAKE_)
			rewriteFutexWake(CI);
		else
			rewriteNonFutexSyscall(CI);
	}

	return PreservedAnalyses::none();
}
