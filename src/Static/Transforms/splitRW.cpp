#include "splitRW.hpp"
#include <llvm/IR/PassManager.h>
#include <llvm/IR/Instructions.h>
#include <llvm/IR/Constants.h>
#include <llvm/IR/InstIterator.h>
#include <llvm/Passes/PassBuilder.h>
#include <llvm/Passes/PassPlugin.h>
#include <llvm/Analysis/ConstantFolding.h>

using namespace llvm;

auto splitRW::run(Function &F, FunctionAnalysisManager &FAM) -> PreservedAnalyses
{
    SmallVector<Instruction *, 16> dead;
    bool changed = false;

    for (Instruction &I : instructions(F)) {
      auto *EVI = dyn_cast<ExtractValueInst>(&I);
      if (!EVI) continue;

      Value *Agg = EVI->getAggregateOperand();
      auto *C = dyn_cast<Constant>(Agg);
      if (!C) continue;

      if (Constant *Folded = ConstantFoldExtractValueInstruction(C, EVI->getIndices())) {
        EVI->replaceAllUsesWith(Folded);
        dead.push_back(EVI);
        changed = true;
      }
    }

    for (Instruction *I : dead) I->eraseFromParent();
    return changed ? PreservedAnalyses::none() : PreservedAnalyses::all();
  
}
