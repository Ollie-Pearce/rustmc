#include "SplitCrossWordAccessesPass.hpp"
#include "../../Error.hpp"
#include <llvm/IR/IRBuilder.h>
#include <llvm/IR/InstIterator.h>
#include <llvm/IR/Instructions.h>
#include <llvm/ADT/SmallVector.h>

using namespace llvm;

/* Coherence tracker word size: 2^3 = 8 bytes */
static constexpr unsigned WordBytes = 8;

/*
 * An access of S bytes with alignment A can cross an 8-byte word
 * boundary iff S > min(A, 8).  The worst-case byte offset within
 * a word for a pointer with alignment A (< 8) is (8 - A), and the
 * access spans bytes [8-A, 8-A+S-1], crossing iff 8-A+S-1 >= 8,
 * i.e. S > A.  For A >= 8 the offset is always 0, so crossing
 * happens iff S > 8.  Combined: S > min(A, 8).
 */
static auto mightCrossWordBoundary(unsigned sizeBytes, unsigned alignment) -> bool
{
	return sizeBytes > std::min(alignment, WordBytes);
}

/*
 * Split a load into alignment-sized sub-loads and compose the
 * result with zext / shl / or (little-endian).
 */
static auto splitLoad(LoadInst *LI, const DataLayout &DL) -> bool
{
	if (LI->isAtomic() || LI->isVolatile())
		return false;

	Type *origTy = LI->getType();

	/* Only handle scalar types we can round-trip through integers */
	if (!origTy->isIntegerTy() && !origTy->isFloatingPointTy() &&
	    !origTy->isPointerTy())
		return false;

	unsigned totalBytes = DL.getTypeStoreSize(origTy);
	unsigned align = LI->getAlign().value();

	if (!mightCrossWordBoundary(totalBytes, align))
		return false;

	unsigned chunkBytes = std::min(align, WordBytes);
	unsigned totalBits = totalBytes * 8;

	LLVMContext &ctx = LI->getContext();
	IRBuilder<> builder(LI);

	Type *fullIntTy = IntegerType::get(ctx, totalBits);
	Type *i8Ty = Type::getInt8Ty(ctx);
	Type *i64Ty = Type::getInt64Ty(ctx);

	Value *result = Constant::getNullValue(fullIntTy);

	for (unsigned offset = 0; offset < totalBytes; offset += chunkBytes) {
		unsigned thisChunk = std::min(chunkBytes, totalBytes - offset);
		Type *chunkTy = IntegerType::get(ctx, thisChunk * 8);

		Value *ptr = LI->getPointerOperand();
		if (offset > 0)
			ptr = builder.CreateGEP(i8Ty, ptr,
				ConstantInt::get(i64Ty, offset), "split.gep");

		Value *sub = builder.CreateAlignedLoad(
			chunkTy, ptr, Align(chunkBytes), "split.load");

		Value *ext = builder.CreateZExt(sub, fullIntTy, "split.zext");
		if (offset > 0)
			ext = builder.CreateShl(ext, offset * 8, "split.shl");
		result = builder.CreateOr(result, ext, "split.or");
	}

	/* Convert back to the original type if it was not integer */
	if (origTy->isFloatingPointTy())
		result = builder.CreateBitCast(result, origTy, "split.cast");
	else if (origTy->isPointerTy())
		result = builder.CreateIntToPtr(result, origTy, "split.itp");

	LI->replaceAllUsesWith(result);
	LI->eraseFromParent();
	return true;
}

/*
 * Split a store into alignment-sized sub-stores by decomposing
 * the value with lshr / trunc (little-endian).
 */
static auto splitStore(StoreInst *SI, const DataLayout &DL) -> bool
{
	if (SI->isAtomic() || SI->isVolatile())
		return false;

	Type *valTy = SI->getValueOperand()->getType();

	if (!valTy->isIntegerTy() && !valTy->isFloatingPointTy() &&
	    !valTy->isPointerTy())
		return false;

	unsigned totalBytes = DL.getTypeStoreSize(valTy);
	unsigned align = SI->getAlign().value();

	if (!mightCrossWordBoundary(totalBytes, align))
		return false;

	unsigned chunkBytes = std::min(align, WordBytes);
	unsigned totalBits = totalBytes * 8;

	LLVMContext &ctx = SI->getContext();
	IRBuilder<> builder(SI);

	Type *fullIntTy = IntegerType::get(ctx, totalBits);
	Type *i8Ty = Type::getInt8Ty(ctx);
	Type *i64Ty = Type::getInt64Ty(ctx);

	Value *val = SI->getValueOperand();

	/* Convert to integer if needed */
	if (valTy->isFloatingPointTy())
		val = builder.CreateBitCast(val, fullIntTy, "split.s.cast");
	else if (valTy->isPointerTy())
		val = builder.CreatePtrToInt(val, fullIntTy, "split.s.pti");

	for (unsigned offset = 0; offset < totalBytes; offset += chunkBytes) {
		unsigned thisChunk = std::min(chunkBytes, totalBytes - offset);
		Type *chunkTy = IntegerType::get(ctx, thisChunk * 8);

		Value *chunk = val;
		if (offset > 0)
			chunk = builder.CreateLShr(chunk, offset * 8, "split.s.shr");
		chunk = builder.CreateTrunc(chunk, chunkTy, "split.s.trunc");

		Value *ptr = SI->getPointerOperand();
		if (offset > 0)
			ptr = builder.CreateGEP(i8Ty, ptr,
				ConstantInt::get(i64Ty, offset), "split.s.gep");

		builder.CreateAlignedStore(chunk, ptr, Align(chunkBytes));
	}

	SI->eraseFromParent();
	return true;
}

auto SplitCrossWordAccessesPass::run(Function &F, FunctionAnalysisManager &FAM)
	-> PreservedAnalyses
{
	const DataLayout &DL = F.getParent()->getDataLayout();
	bool modified = false;

	SmallVector<LoadInst *, 16> loadsToSplit;
	SmallVector<StoreInst *, 16> storesToSplit;

	for (auto &I : instructions(F)) {
		if (auto *LI = dyn_cast<LoadInst>(&I)) {
			unsigned size = DL.getTypeStoreSize(LI->getType());
			unsigned align = LI->getAlign().value();
			if (mightCrossWordBoundary(size, align))
				loadsToSplit.push_back(LI);
		} else if (auto *SI = dyn_cast<StoreInst>(&I)) {
			unsigned size = DL.getTypeStoreSize(SI->getValueOperand()->getType());
			unsigned align = SI->getAlign().value();
			if (mightCrossWordBoundary(size, align))
				storesToSplit.push_back(SI);
		}
	}

	for (auto *LI : loadsToSplit)
		modified |= splitLoad(LI, DL);
	for (auto *SI : storesToSplit)
		modified |= splitStore(SI, DL);

	return modified ? PreservedAnalyses::none() : PreservedAnalyses::all();
}
