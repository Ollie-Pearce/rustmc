#include "PromoteMemcpy.hpp"

#include "Support/Error.hpp"
#include "llvm/Support/Regex.h"
#include <llvm/ADT/Twine.h>
#include <llvm/Analysis/AliasAnalysis.h>
#include <llvm/Analysis/InstructionSimplify.h>
#include <llvm/IR/BasicBlock.h>
#include <llvm/IR/DebugInfo.h>
#include <llvm/IR/Function.h>
#include <llvm/IR/IRBuilder.h>
#include <llvm/IR/InstIterator.h>
#include <llvm/IR/InstrTypes.h>
#include <llvm/IR/Instruction.h>
#include <llvm/IR/Instructions.h>
#include <llvm/IR/Metadata.h>
#include <llvm/IR/Module.h>
#include <llvm/IR/PassManager.h>
#include <llvm/IR/Type.h>

#include <ranges>

using namespace llvm;

void removePromoted(std::ranges::input_range auto &&promoted)
{
	for (auto *MI : promoted) {
		BitCastInst *dst = dyn_cast<BitCastInst>(MI->getRawDest());
		BitCastInst *src = nullptr;
		if (auto *MC = dyn_cast<MemCpyInst>(MI))
			src = dyn_cast<BitCastInst>(MC->getRawSource());

		MI->eraseFromParent();
		if (dst && dst->hasNUses(0))
			dst->eraseFromParent();
		if (src && src->hasNUses(0))
			src->eraseFromParent();
	}
}

auto promoteI32(MemCpyInst *MI, SmallVector<llvm::MemIntrinsic *, 8> &promoted)
{
	Type *I64Type = Type::getInt64Ty(MI->getContext());
	Type *I32Type = Type::getInt32Ty(MI->getContext());
	Type *I8Type = Type::getInt8Ty(MI->getContext());
	auto *nullInt = Constant::getNullValue(I64Type);
	std::vector<Value *> args = {nullInt};
	IRBuilder<> builder(MI);

	Value *promote1xI32_load0 =
		builder.CreateLoad(I32Type, MI->getSource(), "promote1xI32_load");
	builder.CreateStore(promote1xI32_load0, MI->getDest());

	promoted.push_back(MI);
}

auto promoteI16(MemCpyInst *MI, SmallVector<llvm::MemIntrinsic *, 8> &promoted)
{

	Type *I16Type = Type::getInt16Ty(MI->getContext());
	Type *I8Type = Type::getInt8Ty(MI->getContext());
	IRBuilder<> builder(MI);

	Value *promote1xI16_load0 =
		builder.CreateLoad(I16Type, MI->getSource(), "promote1xI32_load");
	builder.CreateStore(promote1xI16_load0, MI->getDest());

	promoted.push_back(MI);
}

auto promoteByteWise(MemCpyInst *MI, SmallVector<llvm::MemIntrinsic *, 8> &promoted) -> bool
{
	ConstantInt *constLength = llvm::dyn_cast<llvm::ConstantInt>(MI->getLength());
	if (!constLength) {
		WARN_ONCE("memintr-length", "Cannot promote non-constant-length mem intrinsic!"
					    "Skipping...\n");
		return false;
	}

	Value *dest = MI->getRawDest();
	Value *src = MI->getRawSource();
	IRBuilder<> builder(MI);

	Type *int8Type = Type::getInt8Ty(MI->getContext());

	Value *destPtr = builder.CreateBitCast(dest, int8Type->getPointerTo());
	Value *srcPtr = builder.CreateBitCast(src, int8Type->getPointerTo());

	Value *zeroValue = ConstantInt::get(int8Type, 0);

	uint64_t lengthValue = constLength->getZExtValue();

	for (uint64_t i = 0; i < constLength->getZExtValue(); ++i) {
		Value *index = ConstantInt::get(builder.getInt64Ty(), i);

		// Compute pointers for source and destination at offset i.
		Value *destGEP = builder.CreateGEP(int8Type, destPtr, index, "dest_gep");
		Value *srcGEP = builder.CreateGEP(int8Type, srcPtr, index, "src_gep");

		// Load a byte from the source.
		Value *loadedVal = builder.CreateLoad(int8Type, srcGEP, "load_byte");
		// Store the byte into the destination.
		builder.CreateStore(loadedVal, destGEP);
	}

	promoted.push_back(MI);
	return true;
}

auto PromoteI64s(MemCpyInst *MI, SmallVector<llvm::MemIntrinsic *, 8> &promoted) -> bool
{
	Value *dest = MI->getRawDest();
	Value *src = MI->getRawSource();
	Value *length = MI->getLength();
	Type *I64Type = Type::getInt64Ty(MI->getContext());
	auto *nullInt = Constant::getNullValue(I64Type);
	std::vector<Value *> args = {nullInt};
	IRBuilder<> builder(MI);

	Type *int8Type = Type::getInt8Ty(MI->getContext());
	Value *destPtr = builder.CreateBitCast(dest, int8Type->getPointerTo());
	Value *srcPtr = builder.CreateBitCast(src, int8Type->getPointerTo());

	auto *constLength = llvm::dyn_cast<llvm::ConstantInt>(length);
	int size_in_i64s = static_cast<int>(constLength->getZExtValue()) / 8;

	for (uint64_t i = 0; i < size_in_i64s; i++) {
		Value *index = ConstantInt::get(builder.getInt64Ty(), i * 8);

		// Compute pointers for source and destination at offset i.
		Value *srcGEP =
			builder.CreateGEP(int8Type, MI->getSource(), index, "new_promote_src_gep");
		Value *destGEP =
			builder.CreateGEP(int8Type, MI->getDest(), index, "new_promote_dst_gep");

		Value *loadedVal = builder.CreateLoad(I64Type, srcGEP, "new_promote_load");
		builder.CreateStore(loadedVal, destGEP);
	}
	promoted.push_back(MI);
	return true;
}

bool IsDivisibleBy8(MemCpyInst *MI, SmallVector<llvm::MemIntrinsic *, 8> &promoted)
{

	ConstantInt *constLength = llvm::dyn_cast<llvm::ConstantInt>(MI->getLength());
	if (!constLength) {
		WARN_ONCE("memintr-length", "Cannot promote non-constant-length mem intrinsic!"
					    "Skipping...\n");
		return false;
	}
	auto lengthint = constLength->getZExtValue();

	switch (lengthint % 8) {
	case 0:
		PromoteI64s(MI, promoted);
		return true;
	case 4:
		promoteI32(MI, promoted);
		return true;
	case 2:
		promoteI16(MI, promoted);
		return true;
	default:
		return false;
	}
	return false;
}

auto promote_8len_memset(MemSetInst *MI, SmallVector<llvm::MemIntrinsic *, 8> &promoted)
{

	Type *I64Type = Type::getInt64Ty(MI->getContext());
	Type *I32Type = Type::getInt32Ty(MI->getContext());
	Type *I8Type = Type::getInt8Ty(MI->getContext());
	auto *nullInt = Constant::getNullValue(I64Type);
	std::vector<Value *> args = {nullInt};
	IRBuilder<> builder(MI);

	auto *dst = MI->getDest();

	auto *argVal = MI->getValue();

	long int ival = dyn_cast<ConstantInt>(argVal)->getSExtValue();
	Value *val = Constant::getIntegerValue(I64Type, APInt(64, ival));

	builder.CreateStore(val, dst, "mynewstoretuna");

	promoted.push_back(MI);

	return true;
}

auto PromoteMemcpy::run(Function &F, FunctionAnalysisManager &FAM) -> PreservedAnalyses
{
	auto modified = false;
	SmallVector<llvm::MemIntrinsic *, 8> promoted;

	for (auto &I : instructions(F)) {
		if (auto *MI = dyn_cast<MemCpyInst>(&I)) {
			if (IsDivisibleBy8(MI, promoted)) {
				modified = true;
			} else {
				modified |= promoteByteWise(MI, promoted);
			}
		}
		if (auto *MS = dyn_cast<MemSetInst>(&I)) {
			modified |= promote_8len_memset(MS, promoted);
		}
	}
	removePromoted(promoted);
	return modified ? PreservedAnalyses::none() : PreservedAnalyses::all();
}
