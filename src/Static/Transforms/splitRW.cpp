#include "splitRW.hpp"
#include <llvm/IR/PassManager.h>
#include <llvm/IR/Instructions.h>
#include <llvm/IR/Constants.h>
#include <llvm/IR/InstIterator.h>
#include <llvm/Passes/PassBuilder.h>
#include <llvm/Passes/PassPlugin.h>
#include <llvm/Analysis/ConstantFolding.h>

using namespace llvm;


static Value *foldExtractOverInsert(ExtractValueInst *EVI, InsertValueInst *IVI) {
  ArrayRef<unsigned> Ext = EVI->getIndices();
  ArrayRef<unsigned> Ins = IVI->getIndices();

  // If the extract path starts with the insert path, we’re reading from the inserted value.
  if (Ext.size() >= Ins.size() &&
      std::equal(Ins.begin(), Ins.end(), Ext.begin())) {
    Value *Inserted = IVI->getInsertedValueOperand();

    if (Ext.size() == Ins.size())
      return Inserted; // exact element inserted

    // Need a deeper extract from the inserted value.
    ArrayRef<unsigned> Suffix = Ext.drop_front(Ins.size());

    if (auto *CIns = dyn_cast<Constant>(Inserted))
      if (Constant *CF = ConstantFoldExtractValueInstruction(CIns, Suffix))
        return CF;

    return ExtractValueInst::Create(Inserted, Suffix, EVI->getName(), EVI);
  }

  // Otherwise, we’re reading from a disjoint part of the base aggregate.
  Value *Base = IVI->getAggregateOperand();

  if (auto *CBase = dyn_cast<Constant>(Base))
    if (Constant *CF = ConstantFoldExtractValueInstruction(CBase, Ext))
      return CF;

  return ExtractValueInst::Create(Base, Ext, EVI->getName(), EVI);
}

auto splitRW::run(Function &F, FunctionAnalysisManager &FAM) -> PreservedAnalyses
{
 SmallVector<Instruction *, 16> Dead;
  bool Changed = false;

  for (Instruction &I : instructions(F)) {
    auto *EVI = dyn_cast<ExtractValueInst>(&I);
    if (!EVI) continue;

    Value *Agg = EVI->getAggregateOperand();

    // 1) extractvalue from literal/constant aggregate
    if (auto *CAgg = dyn_cast<Constant>(Agg)) {
      if (Constant *Folded = ConstantFoldExtractValueInstruction(CAgg, EVI->getIndices())) {
        EVI->replaceAllUsesWith(Folded);
        Dead.push_back(EVI);
        Changed = true;
        continue;
      }
    }

    // 2) extractvalue over insertvalue
    if (auto *IVI = dyn_cast<InsertValueInst>(Agg)) {
      // 2a) Try to fold the insert itself to a constant aggregate first.
      if (auto *BaseC = dyn_cast<Constant>(IVI->getAggregateOperand()))
        if (auto *InsC = dyn_cast<Constant>(IVI->getInsertedValueOperand()))
          if (Constant *FoldedAgg =
                  ConstantFoldInsertValueInstruction(BaseC, InsC, IVI->getIndices()))
            if (Constant *FoldedElt =
                    ConstantFoldExtractValueInstruction(FoldedAgg, EVI->getIndices())) {
              EVI->replaceAllUsesWith(FoldedElt);
              Dead.push_back(EVI);
              Changed = true;
              continue;
            }

      // 2b) Structural rewrite when constants are not available.
      if (Value *V = foldExtractOverInsert(EVI, IVI)) {
        EVI->replaceAllUsesWith(V);
        Dead.push_back(EVI);
        Changed = true;
        continue;
      }
    }

    // 3) extract from undef/poison constant aggregates
    if (isa<UndefValue>(Agg) || isa<PoisonValue>(Agg)) {
      Value *Rep = isa<UndefValue>(Agg) ? UndefValue::get(EVI->getType())
                                        : PoisonValue::get(EVI->getType());
      EVI->replaceAllUsesWith(Rep);
      Dead.push_back(EVI);
      Changed = true;
      continue;
    }
  }

  for (Instruction *D : Dead) D->eraseFromParent();
  return Changed ? PreservedAnalyses::none() : PreservedAnalyses::all();
}