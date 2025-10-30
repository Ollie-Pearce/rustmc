#include <llvm/ADT/SmallVector.h>
#include <llvm/IR/IntrinsicInst.h>
#include <llvm/IR/Module.h>
#include <llvm/Pass.h>

#include <llvm/Passes/PassBuilder.h>

using namespace llvm;


class splitRW : public PassInfoMixin<splitRW> {
public:
	auto run(Function &F, FunctionAnalysisManager &FAM) -> PreservedAnalyses;
    
};