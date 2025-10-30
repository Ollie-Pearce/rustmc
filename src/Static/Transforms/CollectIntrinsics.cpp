#include "CollectIntrinsics.hpp"
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
#include <llvm/Transforms/Utils/Cloning.h>
#include <llvm/IR/Verifier.h>
#include <string>

#include <ranges>

using namespace llvm;

/*static void replaceReturnsWithAggregate(Function &F, StructType *NewST) {
  SmallVector<ReturnInst*, 8> Rets;
  for (BasicBlock &BB : F)
    if (auto *RI = dyn_cast<ReturnInst>(BB.getTerminator()))
      Rets.push_back(RI);

  for (ReturnInst *RI : Rets) {
    IRBuilder<> B(RI);
    Value *Old = RI->getReturnValue();                    // literal {i64,i64}
    if (!Old) { B.CreateRetVoid(); RI->eraseFromParent(); continue; }

    // Extract two i64s irrespective of producer shape (const, phi, etc.)
    Value *V0 = B.CreateExtractValue(Old, 0);
    Value *V1 = B.CreateExtractValue(Old, 1);

    Value *Agg = UndefValue::get(NewST);
    Agg = B.CreateInsertValue(Agg, V0, 0);
    Agg = B.CreateInsertValue(Agg, V1, 1);

    // Replace with correctly typed return
    B.CreateRet(Agg);
    RI->eraseFromParent();
  }
}*/

auto CollectIntrinsics::run(Function &F, FunctionAnalysisManager &FAM) -> PreservedAnalyses
{
	
    // Check if the return type is exactly literal { i64, i64 }
    Type *retTy = F.getReturnType();
    StructType *retStruct = dyn_cast<StructType>(retTy);
    if (!retStruct || !retStruct->isLiteral() || retStruct->getNumElements() != 2)
      return PreservedAnalyses::all();  // not applicable, preserve all analyses

    // Ensure both elements are i64
    Type *elem0 = retStruct->getElementType(0);
    Type *elem1 = retStruct->getElementType(1);
    if (!(elem0->isIntegerTy(64) && elem1->isIntegerTy(64)))
      return PreservedAnalyses::all();  // not an { i64, i64 } struct

    Module *M = F.getParent();
    LLVMContext &Ctx = M->getContext();
    // Get or create the identified struct type "%temp.i64.struct"
    StructType *identStruct = StructType::getTypeByName(Ctx, "temp.i64.struct");
    if (!identStruct) {
      // Create a new identified struct type and set its body to { i64, i64 }
      identStruct = StructType::create(Ctx, {Type::getInt64Ty(Ctx), Type::getInt64Ty(Ctx)}, "temp.i64.struct");
    }

    // Create new function type with the identified struct return
    FunctionType *oldFuncTy = F.getFunctionType();
    FunctionType *newFuncTy = FunctionType::get(identStruct, oldFuncTy->params(), oldFuncTy->isVarArg());
    Function *newFunc = Function::Create(newFuncTy, F.getLinkage(), F.getAddressSpace(),
                                        F.getName() + ".structret", M);
    // Copy function attributes (except those related to return type) if needed
    newFunc->copyAttributesFrom(&F);
    newFunc->setSubprogram(F.getSubprogram());  // preserve debug info, if any

    // Move basic blocks from F to newFunc
    newFunc->splice(newFunc->begin(), &F);
    // Redirect uses of old arguments to new arguments in moved code
    Function::arg_iterator newArgIt = newFunc->arg_begin();
    for (Argument &oldArg : F.args()) {
      if (!oldArg.use_empty()) {
        // Replace all uses of oldArg (now in newFunc's body) with newFunc's corresponding arg
        oldArg.replaceAllUsesWith(&*newArgIt);
      }
      // Copy argument name
      newArgIt->setName(oldArg.getName());
      ++newArgIt;
    }

    // Fix all return instructions in newFunc
    SmallVector<ReturnInst*, 4> rets;
    for (BasicBlock &BB : *newFunc) {
      if (ReturnInst *ret = dyn_cast<ReturnInst>(BB.getTerminator())) {
        rets.push_back(ret);
      }
    }
    for (ReturnInst *ret : rets) {
      Value *origRetVal = ret->getReturnValue(); // Value of type {i64,i64}
      if (!origRetVal) continue; // (skip if void, though in our case it won't be void)
      IRBuilder<> builder(ret);
      // Extract the two i64 values
      Value *lo = builder.CreateExtractValue(origRetVal, 0, "ret.lo");
      Value *hi = builder.CreateExtractValue(origRetVal, 1, "ret.hi");
      // Build identified struct aggregate
      Value *newAgg = UndefValue::get(identStruct);
      newAgg = builder.CreateInsertValue(newAgg, lo, 0);
      newAgg = builder.CreateInsertValue(newAgg, hi, 1);
      // Update return to return the new aggregate
      ret->setOperand(0, newAgg);
    }

    // Old function becomes a wrapper that calls newFunc
    BasicBlock *entryBB = BasicBlock::Create(Ctx, "entry.wrap", &F);
    IRBuilder<> builder(entryBB);
    // Forward all original arguments to the new function call
    SmallVector<Value*, 8> callArgs;
    for (auto &arg : F.args()) {
      callArgs.push_back(&arg);
    }
    CallInst *callNew = builder.CreateCall(newFunc, callArgs, "call_new");
    // Extract the result and repack into { i64, i64 }
    Value *res_lo = builder.CreateExtractValue(callNew, 0, "res.lo");
    Value *res_hi = builder.CreateExtractValue(callNew, 1, "res.hi");
    Value *retStructVal = UndefValue::get(retStruct); // old literal struct type
    retStructVal = builder.CreateInsertValue(retStructVal, res_lo, 0);
    retStructVal = builder.CreateInsertValue(retStructVal, res_hi, 1);
    builder.CreateRet(retStructVal);

    // Optionally adjust linkage: newFunc can be made internal if F was externally visible
    if (!F.isDeclaration()) {
      newFunc->setLinkage(GlobalValue::InternalLinkage);
    }

    // The original function F now has a new body and remains to satisfy callers.
    // We do not delete F or update call sites.

    return PreservedAnalyses::none();  // IR was changed
}