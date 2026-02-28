#include "FoldInterpreterUnsupportedConstants.hpp"
#include <llvm/IR/PassManager.h>
#include <llvm/IR/Instructions.h>
#include <llvm/IR/Constants.h>
#include <llvm/IR/InstIterator.h>
#include <llvm/Passes/PassBuilder.h>
#include <llvm/Passes/PassPlugin.h>
#include <llvm/Analysis/ConstantFolding.h>

using namespace llvm;

// Recursively expand a constant aggregate into a chain of insertvalue instructions.
// Returns the final Value representing the fully constructed aggregate.
// InsertPt is the instruction before which to insert new instructions.
static Value *expandConstantAggregate(Constant *C, Instruction *InsertPt) {
  Type *Ty = C->getType();

  // Only handle struct and array types
  if (!Ty->isStructTy() && !Ty->isArrayTy()) {
    return C;
  }

  // Don't expand undef/poison - they're fine as-is
  if (isa<UndefValue>(C) || isa<PoisonValue>(C)) {
    return C;
  }

  // Don't expand zeroinitializer - the interpreter handles this
  if (isa<ConstantAggregateZero>(C)) {
    return C;
  }

  // Get the number of elements
  unsigned NumElems;
  if (auto *STy = dyn_cast<StructType>(Ty))
    NumElems = STy->getNumElements();
  else if (auto *ATy = dyn_cast<ArrayType>(Ty))
    NumElems = ATy->getNumElements();
  else {
    return C;
  }

  // Start with poison as the base
  Value *Agg = PoisonValue::get(Ty);

  // Insert each element - we must create InsertValueInst directly to prevent
  // constant folding that IRBuilder would do
  for (unsigned i = 0; i < NumElems; ++i) {
    Constant *Elem = C->getAggregateElement(i);
    if (!Elem) {
      return C; // Can't expand, return original
    }

    // Recursively expand nested aggregates
    Value *ExpandedElem = expandConstantAggregate(Elem, InsertPt);

    // Create InsertValueInst directly (not through IRBuilder) to prevent constant folding
    auto *IVI = InsertValueInst::Create(Agg, ExpandedElem, {i}, "", InsertPt);
    Agg = IVI;
  }

  return Agg;
}

// Check if a constant needs expansion (is a non-trivial aggregate)
static bool needsExpansion(Constant *C) {
  Type *Ty = C->getType();

  if (!Ty->isStructTy() && !Ty->isArrayTy())
    return false;

  // Don't need to expand undef/poison/zeroinitializer
  if (isa<UndefValue>(C) || isa<PoisonValue>(C) || isa<ConstantAggregateZero>(C))
    return false;

  // ConstantStruct and ConstantArray need expansion
  if (isa<ConstantStruct>(C) || isa<ConstantArray>(C))
    return true;

  // ConstantDataArray/ConstantDataVector are typically fine, but check elements
  // to be safe - if they contain nested aggregates, we need to expand
  if (auto *CDA = dyn_cast<ConstantDataSequential>(C))
    return false; // These are fine, just primitive data

  return false;
}

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


auto FoldInterpreterUnsupportedConstants::run(Function &F, FunctionAnalysisManager &FAM) -> PreservedAnalyses
{
  SmallVector<Instruction *, 16> Dead;
  bool Changed = false;

  // First pass: handle extractvalue instructions
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
  Dead.clear();

  // Second pass: handle return instructions with constant aggregates
  for (BasicBlock &BB : F) {
    auto *RI = dyn_cast<ReturnInst>(BB.getTerminator());
    if (!RI || !RI->getReturnValue())
      continue;

    auto *C = dyn_cast<Constant>(RI->getReturnValue());
    if (!C || !needsExpansion(C))
      continue;

    // Expand the constant aggregate into insertvalue chain
    Value *Expanded = expandConstantAggregate(C, RI);

    if (Expanded != C) {
      RI->setOperand(0, Expanded);
      Changed = true;
    }
  }

  // Third pass: handle insertvalue instructions with constant aggregate bases
  // This handles cases like: insertvalue { i64, i64 } { i64 8, i64 poison }, i64 %val, 1
  for (Instruction &I : instructions(F)) {
    auto *IVI = dyn_cast<InsertValueInst>(&I);
    if (!IVI) continue;

    auto *BaseC = dyn_cast<Constant>(IVI->getAggregateOperand());
    if (!BaseC || !needsExpansion(BaseC))
      continue;

    // Expand the constant base into insertvalue chain
    Value *ExpandedBase = expandConstantAggregate(BaseC, IVI);

    if (ExpandedBase != BaseC) {
      IVI->setOperand(0, ExpandedBase);
      Changed = true;
    }
  }

  return Changed ? PreservedAnalyses::none() : PreservedAnalyses::all();
}
