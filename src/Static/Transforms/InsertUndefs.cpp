#include "InsertUndefs.hpp"

#include "../../Error.hpp"
#include <llvm/ADT/Twine.h>
#include <llvm/Analysis/InstructionSimplify.h>
#include <llvm/IR/BasicBlock.h>
#include <llvm/IR/DebugInfo.h>
#include <llvm/IR/Function.h>
#include <llvm/IR/IRBuilder.h>
#include <llvm/IR/InstIterator.h>
#include <llvm/IR/InstrTypes.h>
#include <llvm/IR/Instruction.h>
#include <llvm/IR/Instructions.h>
#include <llvm/IR/Module.h>
#include <llvm/IR/PassManager.h>
#include <llvm/IR/Type.h>

#include <ranges>

using namespace llvm;

auto InsertStoreUndef(AllocaInst *AI, uint64_t NumElems) -> bool
{
	IRBuilder<> Builder(AI->getNextNode());

	Type *I8Ty = Type::getInt8Ty(AI->getContext());

	/* Use stores no wider than the alloca alignment to avoid
	 * crossing 8-byte coherence bucket boundaries in MIXER. */
	uint64_t align = AI->getAlign().value();
	uint64_t stepSize = std::min(align, uint64_t(8));
	Type *StoreTy = IntegerType::get(AI->getContext(), stepSize * 8);
	Value *UndefVal = UndefValue::get(StoreTy);

	/* Full-width stores for the aligned portion */
	uint64_t fullEnd = (NumElems / stepSize) * stepSize;
	for (uint64_t i = 0; i < fullEnd; i += stepSize) {
		Value *Indices[1] = {
			ConstantInt::get(Type::getInt64Ty(AI->getContext()), i)
		};
		Value *ElemPtr = Builder.CreateGEP(
			I8Ty, AI, Indices, "elemPtr"
		);
		Builder.CreateStore(UndefVal, ElemPtr);
	}

	/* Byte-by-byte for any remainder */
	Value *UndefI8 = UndefValue::get(I8Ty);
	for (uint64_t i = fullEnd; i < NumElems; i++) {
		Value *Indices[1] = {
			ConstantInt::get(Type::getInt64Ty(AI->getContext()), i)
		};
		Value *ElemPtr = Builder.CreateGEP(
			I8Ty, AI, Indices, "elemPtr"
		);
		Builder.CreateStore(UndefI8, ElemPtr);
	}
	return true;
}

auto InsertUndefs::run(Function &F, FunctionAnalysisManager &FAM) -> PreservedAnalyses
{
	auto modified = false;

	for (auto &I : instructions(F)) {
		if (auto *AI = dyn_cast<AllocaInst>(&I)) {
			if (auto *ArrTy = dyn_cast<ArrayType>(AI->getAllocatedType())) {
				uint64_t NumElems = ArrTy->getNumElements();
				if (NumElems > 0) {
					InsertStoreUndef(AI, NumElems);
					modified = true;
				}
			}
		}
	}
	return modified ? PreservedAnalyses::none() : PreservedAnalyses::all();
}
