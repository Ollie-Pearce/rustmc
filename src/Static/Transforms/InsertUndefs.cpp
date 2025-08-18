#include "InsertUndefs.hpp"

#include "../../Error.hpp"
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
#include <llvm/IR/Module.h>
#include <llvm/IR/PassManager.h>
#include <llvm/IR/Type.h>

#include <ranges>

using namespace llvm;

auto InsertStoreUndef(AllocaInst *AI, uint64_t NumElems) -> bool
{
	IRBuilder<> Builder(AI->getNextNode());

	Type *OpaquePtrTy = PointerType::get(AI->getContext(), 0);

	Type *I8Ty = Type::getInt8Ty(AI->getContext());
    Value *UndefI64 = UndefValue::get(OpaquePtrTy);

	for (uint64_t i = 0; i < NumElems; i = i + 8) {
            // Indices for GEP: [0, i]
            Value *Indices[1] = {
              ConstantInt::get(Type::getInt64Ty(AI->getContext()), i)
            };

            // Create a pointer to the i-th element
            // GEP type is ArrTy, the pointer operand is the alloca
            Value *ElemPtr = Builder.CreateGEP(
                I8Ty,                // Pointee type
                AI,               // Base pointer
                Indices,              // GEP indices
                "elemPtr"
            );

			Builder.CreateStore(UndefI64, ElemPtr);
	}
	return true;
}

auto InsertSingleUndef(AllocaInst *AI) {
	IRBuilder<> Builder(AI->getNextNode());
	Type *OpaquePtrTy = PointerType::get(AI->getContext(), 0);
	Value *UndefI64 = UndefValue::get(OpaquePtrTy);
	Builder.CreateStore(UndefI64, AI);

	return true;
}

auto InsertUndefs::run(Function &F, FunctionAnalysisManager &FAM) -> PreservedAnalyses
{
	auto modified = false;

	int count = 0; 
	for (auto &I : instructions(F)) {
		if (auto *AI = dyn_cast<AllocaInst>(&I)) {
			
			if (auto *ArrTy = dyn_cast<ArrayType>(AI->getAllocatedType())) {
				uint64_t NumElems = ArrTy->getNumElements();

				if (NumElems >= 16 && NumElems % 8 == 0) {
					InsertStoreUndef(AI, NumElems);
					modified = true;
				} else if (NumElems == 8) {
					InsertSingleUndef(AI);
					modified = true;
				}
			}
		}
	}
	return modified ? PreservedAnalyses::none() : PreservedAnalyses::all();
}