#include "PromotePause.hpp"

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

void removePromotedPauses(std::ranges::input_range auto &&promoted)
{
	for (auto *CI : promoted) {
		//errs() << "Erasing: ";
		//CI->dump();

		if (!CI->use_empty()) {
			// Replace uses with undef of matching type
			//errs() << "Use list not empty";
		} else {
			//errs() << "No uses";

			if (auto *V = dyn_cast<Value>(CI)) {
				if (V->hasNUses(0)) {
					//errs() << "No uses, erasing";
					CI->eraseFromParent();
				} else {
					//errs() << "Has uses, not erasing";
				}
			} else {
				//errs() << "Not a value, cannot erase";
			}
		}

	}
}

auto PromotePause::run(Function &F, FunctionAnalysisManager &FAM) -> PreservedAnalyses
{
	int promotecount = 0;
	int potentialpromote = 0;
	auto modified = false;
	int gep_edge_cases = 0;

	SmallVector<llvm::CallInst *, 8> promoted;

	for (auto &I : instructions(F)) {

		if (auto *CI = dyn_cast<CallInst>(&I)){
			if (CI->getCalledFunction()) {
				if (CI->getCalledFunction()->getName() == "llvm.x86.sse2.pause"){
					promoted.push_back(CI);
					modified = true;
				}
			} else {
				//errs() << "Unknown function\n";
			}
		}
		
	}

	removePromotedPauses(promoted);
	return modified ? PreservedAnalyses::none() : PreservedAnalyses::all();
}