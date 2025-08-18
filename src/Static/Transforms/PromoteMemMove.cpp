#include "PromoteMemMove.hpp"

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

void removePromoted3(std::ranges::input_range auto &&promoted)
{
	for (auto *MI : promoted) {
		BitCastInst *dst = dyn_cast<BitCastInst>(MI->getRawDest());
		BitCastInst *src = nullptr;
		if (auto *MC = dyn_cast<MemMoveInst>(MI))
			src = dyn_cast<BitCastInst>(MC->getRawSource());

		MI->eraseFromParent();
		if (dst && dst->hasNUses(0))
			dst->eraseFromParent();
		if (src && src->hasNUses(0))
			src->eraseFromParent();
	}
}

bool promoteMemMove(MemMoveInst *MI, SmallVector<llvm::MemIntrinsic *, 8> &promoted, int length){
	IRBuilder<> builder(MI);
	Type *I64Type = Type::getInt64Ty(MI->getContext());
	Type *I8Type = Type::getInt8Ty(MI->getContext());
	auto *nullInt = Constant::getNullValue(I64Type);

	Value *promote3xI64_load0 = builder.CreateLoad(I64Type, MI->getSource());
	builder.CreateStore(promote3xI64_load0, MI->getDest());

	for(int i = 8; i < length; i += 8){
		
		std::vector<Value *> args = {nullInt};
		
		Value *gepSrc = builder.CreateGEP(I8Type, MI->getSource(), args);
		Value *gepDst = builder.CreateGEP(I8Type, MI->getDest(), args);
		
		Value *loadVal = builder.CreateLoad(I64Type, gepSrc);
		builder.CreateStore(loadVal, gepDst);

		args.pop_back();
		args.push_back(Constant::getIntegerValue(I64Type, APInt(64, i + 8)));
	}
	promoted.push_back(MI);

	return true;

}

bool promote3xI642(MemMoveInst *MI, SmallVector<llvm::MemIntrinsic *, 8> &promoted)
{
	Type *I64Type = Type::getInt64Ty(MI->getContext());
	Type *I32Type = Type::getInt32Ty(MI->getContext());
	Type *I8Type = Type::getInt8Ty(MI->getContext());
	auto *nullInt = Constant::getNullValue(I64Type);
	std::vector<Value *> args = {nullInt};
	IRBuilder<> builder(MI);

	Value *promote3xI64_load0 = builder.CreateLoad(I64Type, MI->getSource());
	builder.CreateStore(promote3xI64_load0, MI->getDest());
	args.pop_back();
	args.push_back(Constant::getIntegerValue(I64Type, APInt(64, 8)));

	Value *promoteS3xI64_srcgep1 = builder.CreateGEP(I8Type, MI->getSource(), args);
	Value *promote3xI64_load1 = builder.CreateLoad(I64Type, promoteS3xI64_srcgep1);
	Value *promote3xI64_dstgep1 = builder.CreateGEP(I8Type, MI->getDest(), args);
	builder.CreateStore(promote3xI64_load1, promote3xI64_dstgep1);
	args.pop_back();
	args.push_back(Constant::getIntegerValue(I64Type, APInt(64, 16)));

	Value *promoteS3xI64_srcgep2 = builder.CreateGEP(I8Type, MI->getSource(), args);
	Value *promote3xI64_load2 = builder.CreateLoad(I64Type, promoteS3xI64_srcgep2);
	Value *promoteS3xI64_dstgep2 = builder.CreateGEP(I8Type, MI->getDest(), args);
	builder.CreateStore(promote3xI64_load2, promoteS3xI64_dstgep2, "promote3xI64_store");

	promoted.push_back(MI);
	return true;
}


bool isSpecial2(MemMoveInst *MI, SmallVector<llvm::MemIntrinsic *, 8> &promoted)
{
	auto *constLength = llvm::dyn_cast<llvm::ConstantInt>(MI->getLength());

	switch (constLength->getZExtValue()) {
	case 24:
		promote3xI642(MI, promoted);
		return true;
	default:
		return false;
	}
	return false;
}


auto PromoteMemMove::run(Function &F, FunctionAnalysisManager &FAM) -> PreservedAnalyses
{
	int promotecount = 0;
	int potentialpromote = 0;
	auto modified = false;
	int gep_edge_cases = 0;

	SmallVector<llvm::MemIntrinsic *, 8> promoted;

	for (auto &I : instructions(F)) {
		if (auto *MI = dyn_cast<MemMoveInst>(&I)) {
			if (isa<Constant>(MI->getLength())){
					auto *constLength = llvm::dyn_cast<llvm::ConstantInt>(MI->getLength());
					errs() << "Promoting MemMove of length: " << constLength->getZExtValue() << "\n";
					promoteMemMove(MI, promoted, constLength->getZExtValue());
					modified = true;
				}
			} 
		
	}
	removePromoted3(promoted);
	return modified ? PreservedAnalyses::none() : PreservedAnalyses::all();
}