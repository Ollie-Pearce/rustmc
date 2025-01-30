#include "splitRW.hpp"

#include "Support/Error.hpp"
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
#include "llvm/Analysis/MemoryDependenceAnalysis.h"

#include <ranges>

using namespace llvm;

void removeStores(std::ranges::input_range auto &&promotedStores)
{
	for (auto *S : promotedStores) {
        //S->eraseFromParent();
        errs() << "----------------------------------------";
        errs() << "Instruction: \n";
        S->dump();
        errs() << "Iterating through uses of store ptr: \n";
        S->getPointerOperand()->dump();
        errs() << "\n";

        if (auto *my_value = dyn_cast<Value>(S->getPointerOperand())) {
                for (auto &use : my_value->uses()) {
                auto *user = use.getUser();

                errs() << "Use: \n";
                user->dump(); 
            }
        }
	}
    errs() << "----------------------------------------";
}

auto isNonI8Int(Value *op) -> bool
{
    if (auto *intType = dyn_cast<IntegerType>(op->getType()))
    {
        return !(intType->isIntegerTy(8) || intType->isIntegerTy(1));
    }else{
        return false;
    }
}

auto promoteMemcpyBytes(MemCpyInst *MI, SmallVector<llvm::MemIntrinsic *, 8> &promoted) -> bool
{
	IRBuilder<> builder(MI);
    Value *src = MI->getSource();
    Value *dst = MI->getDest();
	auto *i64Ty = IntegerType::getInt64Ty(MI->getContext());
	auto *i8Ty = IntegerType::getInt8Ty(MI->getContext());

    uint64_t byteLen = 0;
    Value *SizeValue = MI->getLength();
    if (auto *ConstSize = llvm::dyn_cast<llvm::ConstantInt>(SizeValue)) {
        byteLen = ConstSize->getZExtValue();
    } else {
        errs() << "Could not cast length to constant";
        return false;
    }
	promoted.push_back(MI); 
	return true;
}

auto splitStore(StoreInst *storeI, Value *storedVal, unsigned bitWidth, SmallVector<llvm::StoreInst *, 8> &promotedStores) -> bool
{
    IRBuilder<> builder(storeI);

    Value *val = storedVal;
    auto *i64Ty = IntegerType::getInt64Ty(storedVal->getContext());
	auto *nullInt = Constant::getNullValue(i64Ty);
    uint64_t shiftAmount;

    std::vector<Type *> structFields;
    structFields.push_back(Type::getInt8Ty(storedVal->getContext()));
    structFields.push_back(Type::getInt8Ty(storedVal->getContext()));
    structFields.push_back(Type::getInt8Ty(storedVal->getContext()));
    structFields.push_back(Type::getInt8Ty(storedVal->getContext()));
    structFields.push_back(Type::getInt8Ty(storedVal->getContext()));
    structFields.push_back(Type::getInt8Ty(storedVal->getContext()));
    structFields.push_back(Type::getInt8Ty(storedVal->getContext()));
    structFields.push_back(Type::getInt8Ty(storedVal->getContext()));
     
    StructType *myStructType = StructType::create(storedVal->getContext(), structFields, "MyStruct");



    std::vector<Value *> args;

    std::vector<Value *> bytes(8);
    for (int i = 0; i < (bitWidth / 8); ++i) {
        args.push_back(Constant::getIntegerValue(i64Ty, APInt(64, i)));
        shiftAmount = i * 8;
            //Value *shiftAmount = ConstantInt::get(Type::getInt64Ty(storedVal->getContext()), i * 8);
        Value *shifted = builder.CreateLShr(val, shiftAmount, "shifted"); //Right shift, second arg needs to be uint_64t
        //errs() << "shifted";
        //shifted->dump();
        bytes[i] = builder.CreateTrunc(shifted, Type::getInt8Ty(storedVal->getContext()), "byte");
        //errs() << "bytes";
        //bytes[i]->dump();
        Value *noni8GEP = builder.CreateInBoundsGEP(Type::getInt8Ty(storedVal->getContext()), storeI->getOperand(1), args);
        builder.CreateStore(bytes[i], noni8GEP);
        args.pop_back();
    }

    promotedStores.push_back(storeI);
    return true;
}

auto splitLoads(LoadInst *loadI, unsigned bitWidth) {
    IRBuilder<> builder(loadI);

    int x = 0;
    std::vector<Value *> bytes(8);
    for (int i = 0; i < (bitWidth / 8); ++i) {
        x = x + 1;
    }
    return true;
}

auto splitRW::run(Function &F, FunctionAnalysisManager &FAM) -> PreservedAnalyses
{
	int promotecount = 0;
	int potentialpromote = 0;
	auto modified = false;

    auto &MDR = FAM.getResult<MemoryDependenceAnalysis>(F);

    // Iterate over all instructions in the function
    for (auto &I : instructions(F)) {
        if (auto *Load = dyn_cast<LoadInst>(&I)) {
            // Get the memory dependency for the load
            auto DepResult = MDR.getDependency(Load);

            // Check the type of dependency
            if (DepResult.isNonLocal()) {
                errs() << "Non-local dependency detected for: " << *Load << "\n";
            } else if (auto *DepInst = DepResult.getInst()) {
                // Dependency is an instruction, check if it's a store
                if (!isa<StoreInst>(DepInst)) {
                    errs() << "Uninitialized read detected at: " << *Load << "\n";
                    errs() << "Dependency instruction: " << *DepInst << "\n";
                }
            } else {
                // No dependency found, indicating a potential uninitialized read
                errs() << "Uninitialized read detected at: " << *Load << "\n";
            }
        }
    }
	return modified ? PreservedAnalyses::none() : PreservedAnalyses::all();
}
