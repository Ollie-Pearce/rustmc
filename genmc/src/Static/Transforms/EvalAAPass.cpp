#include "EvalAAPass.hpp"

#include <llvm/Analysis/AliasAnalysis.h>
#include <llvm/Analysis/MemoryLocation.h>
#include <llvm/IR/BasicBlock.h>
#include <llvm/IR/Function.h>
#include <llvm/IR/InstIterator.h>
#include <llvm/IR/Instructions.h>
#include <llvm/IR/Module.h>
#include <llvm/IR/PassManager.h>
#include <llvm/Support/raw_ostream.h>

using namespace llvm;

auto identify_alias(Module &M, ModuleAnalysisManager &MAM, int alias_results[4])
{

    auto &FAM = MAM.getResult<FunctionAnalysisManagerModuleProxy>(M).getManager();
	// Get the alias analysis result from the FunctionAnalysisManager
	

	for (auto &F : M) {
        auto &AA = FAM.getResult<AAManager>(F);
		for (auto &BB : F) {
			for (auto &I : BB) {
				if (auto *loadi = dyn_cast<LoadInst>(&I)) {
					// Compare this load with every other memory operation
					// (loads/stores)
					for (auto &OtherBB : F) {
						for (auto &OtherI : OtherBB) {
							if (auto *OtherLoadInst =
								    dyn_cast<LoadInst>(&OtherI)) {
                                        AliasResult result = AA.alias(MemoryLocation::get(loadi), MemoryLocation::get(OtherLoadInst));
								switch (result) {
                                case AliasResult::MustAlias:
									alias_results[3] =
										alias_results[3] +
										1;
									break;
								case AliasResult::NoAlias:
									alias_results[0] =
										alias_results[0] +
										1;
									break;
								case AliasResult::MayAlias:
									alias_results[1] =
										alias_results[1] +
										1;
									break;
								case AliasResult::PartialAlias:
									alias_results[2] =
										alias_results[2] +
										1;
									break;
								
								}
							}
						}
					}
				}
			}
		}
		// Iterate over the instructions in the function

		// Preserve analyses since we are not modifying the function
		
	}
    return PreservedAnalyses::all();
}

auto EvalAAPass::run(llvm::Module &M, llvm::ModuleAnalysisManager &MAM) -> PreservedAnalyses
{

	/* Scan through the instructions and lower intrinsic calls */
	auto modified = false;

	int alias_results[4] = {0, 0, 0, 0};

	identify_alias(M, MAM, alias_results);

	for (int i = 0; i < 4; ++i) {
		switch (i) {
		case 0:
			errs() << "\n No alias: " << alias_results[i];
			break;
		case 1:
			errs() << "\n May alias: " << alias_results[i];
			break;
		case 2:
			errs() << "\n Partial alias: " << alias_results[i];
			break;
		case 3:
			errs() << "\n Must alias: " << alias_results[i];
			break;
		}
	}

	return modified ? PreservedAnalyses::none() : PreservedAnalyses::all();
}
