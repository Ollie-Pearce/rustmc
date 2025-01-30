
#include <llvm/Passes/PassBuilder.h>

using namespace llvm;

class EvalAAPass : public PassInfoMixin<EvalAAPass> {
public:
	auto run(Module &F, ModuleAnalysisManager &FAM) -> PreservedAnalyses;
};
