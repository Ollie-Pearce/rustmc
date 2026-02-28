#include <llvm/ADT/SmallVector.h>
#include <llvm/IR/IntrinsicInst.h>
#include <llvm/IR/Module.h>
#include <llvm/Pass.h>

#include <llvm/Passes/PassBuilder.h>

using namespace llvm;

class FoldInterpreterUnsupportedConstants : public PassInfoMixin<FoldInterpreterUnsupportedConstants >{
public:
	auto run(Function &F, FunctionAnalysisManager &FAM) -> PreservedAnalyses;
    
};
