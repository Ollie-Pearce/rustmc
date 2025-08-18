#include <llvm/ADT/SmallVector.h>
#include <llvm/IR/IntrinsicInst.h>
#include <llvm/IR/Module.h>
#include <llvm/Pass.h>

#include <llvm/Passes/PassBuilder.h>

using namespace llvm;


class CollectIntrinsics : public PassInfoMixin<CollectIntrinsics> {
public:
	auto run(Function &F, FunctionAnalysisManager &FAM) -> PreservedAnalyses;
    
};