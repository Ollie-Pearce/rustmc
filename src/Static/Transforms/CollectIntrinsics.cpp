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
#include <string>

#include <ranges>

using namespace llvm;

auto CollectIntrinsics::run(Function &F, FunctionAnalysisManager &FAM) -> PreservedAnalyses
{
	int promotecount = 0;
	int potentialpromote = 0;
	auto modified = false;
	int gep_edge_cases = 0;

	SmallVector<llvm::CallInst *, 8> promoted;

	for (auto &I : instructions(F)) {

		if (auto *CI = dyn_cast<CallInst>(&I)){
			if (CI->getCalledFunction()) {
                if (CI->getCalledFunction()->getName().find("llvm.") != std::string::npos){
                    errs() << "Found LLVM intrinsic: " << CI->getCalledFunction()->getName() << "\n";
                }
			} else {
				errs() << "Unknown function\n";
			}
		}
		
	}

	return modified ? PreservedAnalyses::none() : PreservedAnalyses::all();
}