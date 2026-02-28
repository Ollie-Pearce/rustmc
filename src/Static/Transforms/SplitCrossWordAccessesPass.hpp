#ifndef GENMC_SPLIT_CROSS_WORD_ACCESSES_PASS_HPP
#define GENMC_SPLIT_CROSS_WORD_ACCESSES_PASS_HPP

#include <llvm/Passes/PassBuilder.h>

using namespace llvm;

class SplitCrossWordAccessesPass : public PassInfoMixin<SplitCrossWordAccessesPass> {
public:
	auto run(Function &F, FunctionAnalysisManager &FAM) -> PreservedAnalyses;
};

#endif
