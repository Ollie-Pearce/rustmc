#include "PromoteMemcpy.hpp"
// #include "MyModuleAnalysisPass.hpp"
#include "../../Error.hpp"
#include "llvm/BinaryFormat/Dwarf.h"
#include "llvm/Support/Regex.h"
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
#include <llvm/IR/Metadata.h>
#include <llvm/IR/Module.h>
#include <llvm/IR/PassManager.h>
#include <llvm/IR/Type.h>
#include <unordered_set>

#include <ranges>

using namespace llvm;

struct VarFieldInfo {
	std::string VariableName;
	std::vector<std::pair<uint64_t, uint64_t>>
		FieldSizesAndOffsets; // Smallest field sizes in bits
};

void removePromoted(std::ranges::input_range auto &&promoted)
{
	for (auto *MI : promoted) {
		BitCastInst *dst = dyn_cast<BitCastInst>(MI->getRawDest());
		BitCastInst *src = nullptr;
		if (auto *MC = dyn_cast<MemCpyInst>(MI))
			src = dyn_cast<BitCastInst>(MC->getRawSource());

		// errs() << "Erasing: ";
		// MI->dump();
		MI->eraseFromParent();
		if (dst && dst->hasNUses(0))
			dst->eraseFromParent();
		if (src && src->hasNUses(0))
			src->eraseFromParent();
	}
}

auto promoteI64Memset(MemSetInst *MI, SmallVector<llvm::MemIntrinsic *> &promoted)
{

	Type *I64Type = Type::getInt64Ty(MI->getContext());
	Type *I32Type = Type::getInt32Ty(MI->getContext());
	Type *I8Type = Type::getInt8Ty(MI->getContext());
	auto *nullInt = Constant::getNullValue(I64Type);
	std::vector<Value *> args = {nullInt};
	IRBuilder<> builder(MI);

	auto *dst = MI->getDest();

	auto *argVal = MI->getValue();

	auto *memset_val = MI->getArgOperand(1);

	if (auto *memset_val_const = dyn_cast<ConstantInt>(memset_val)) {
		if (auto *memset_length_const = dyn_cast<ConstantInt>(MI->getLength())) {

			for (uint64_t i = 0; i < memset_length_const->getZExtValue(); i++) {
				Value *offset = ConstantInt::get(I64Type, i);
				Value *addr = builder.CreateInBoundsGEP(
					builder.getInt8Ty(), dst, offset, "memset_testfish_.gep");
				builder.CreateStore(memset_val_const, dst);
			}
		} else {
			errs() << " \n Cannot promote memset with non-constant length: ";
			MI->dump();
			return false;
		}

		promoted.push_back(MI); // Maybe move this to the end of the function?
	} else {
		errs() << " \n Cannot promote memset with non-constant value: ";
		MI->dump();
		abort();
		return false;
	}

	return true;
}

void convertMemcpyWithDebugInfo(MemCpyInst *MemCpyToConvert, VarFieldInfo TypeInfo,
				SmallVector<llvm::MemIntrinsic *> &promoted)
{
	bool trailingGap = false;
	std::vector<std::pair<uint64_t, uint64_t>> uninitialisedFields;

	IRBuilder<> builder(MemCpyToConvert);
	LLVMContext &ctx = MemCpyToConvert->getContext();

	Type *int8Type = Type::getInt8Ty(ctx);
	errs() << "\n Converting memcpy with debug info\n";
	MemCpyToConvert->dump();
	errs() << "\n From function: " << MemCpyToConvert->getFunction()->getName();

	int total = 0;
	for (auto &pair : TypeInfo.FieldSizesAndOffsets) {
		total += pair.first;
	}
	ConstantInt *constLength = llvm::dyn_cast<llvm::ConstantInt>(MemCpyToConvert->getLength());
	if (constLength) {
		if (total != constLength->getZExtValue()) {
			errs() << "\n Total size mismatch: " << total << " vs "
			       << constLength->getZExtValue();

			// Identify gaps
			errs() << "\n Identifying gaps:\n";
			uint64_t previousEnd = 0;
			for (auto &pair : TypeInfo.FieldSizesAndOffsets) {
				uint64_t offset = pair.second;
				uint64_t size = pair.first;
				if (offset > previousEnd) {
					errs() << " Gap from " << previousEnd << " to " << offset
					       << "\n";
					uninitialisedFields.push_back(
						std::make_pair(previousEnd, offset));
				}
				previousEnd = offset + size;
			}
			if (previousEnd < constLength->getZExtValue()) {
				errs() << "Traiing gap from " << previousEnd << " to "
				       << constLength->getZExtValue() << "\n";
				uninitialisedFields.push_back(
					std::make_pair(previousEnd, constLength->getZExtValue()));
			}
		}
	}

	for (int i = 0; i < TypeInfo.FieldSizesAndOffsets.size(); i++) {
		if (TypeInfo.FieldSizesAndOffsets[i].first ==
		    0) // Skip zero sized fields, e.g. phantomdata
		{
			continue;
		}

		errs() << "\n"
		       << " constructing index for field offset: "
		       << (TypeInfo.FieldSizesAndOffsets[i].second * 8)
		       << "with size: " << (TypeInfo.FieldSizesAndOffsets[i].first * 8);

		Value *index = ConstantInt::get(builder.getInt64Ty(),
						(TypeInfo.FieldSizesAndOffsets[i].second));

		Type *TypeToInsert =
			IntegerType::get(ctx, TypeInfo.FieldSizesAndOffsets[i].first * 8);

		Value *srcGEP = builder.CreateGEP(int8Type, MemCpyToConvert->getSource(), index,
						  "debug_src_gep");
		Value *loadedVal = builder.CreateLoad(TypeToInsert, srcGEP, "debug_load");
		Value *destGEP = builder.CreateGEP(int8Type, MemCpyToConvert->getDest(), index,
						   "debug_dest_gep");
		builder.CreateStore(loadedVal, destGEP);
	}

	for (int i = 0; i < uninitialisedFields.size(); i++) {
		if ((uninitialisedFields[i].second - uninitialisedFields[i].first) % 8 == 0) {
			for (int j = 0;
			     j < (uninitialisedFields[i].second - uninitialisedFields[i].first);
			     j = j + 8) {
				Value *index = ConstantInt::get(builder.getInt64Ty(),
								uninitialisedFields[i].first + j);
				errs() << "Generating i64 load/store pairs for index: "
				       << (uninitialisedFields[i].first + j) << "\n";
				Type *TypeToInsert = IntegerType::get(ctx, 64);

				Value *srcGEP = builder.CreateGEP(int8Type,
								  MemCpyToConvert->getSource(),
								  index, "trailing_gap_src_gep");
				Value *loadedVal = builder.CreateLoad(TypeToInsert, srcGEP,
								      "trailing_gap_load");
				Value *destGEP = builder.CreateGEP(int8Type,
								   MemCpyToConvert->getDest(),
								   index, "trailing_gap_dest_gep");
				builder.CreateStore(loadedVal, destGEP);
			}
		}
	}

	errs() << "\n "
		  "--------------------------------------------------------------------------------"
		  "--------------- \n";

	promoted.push_back(MemCpyToConvert);

	return;
}

// Helper function to collect field sizes and offsets recursively
void collectFieldSizesAndOffsets(DIDerivedType *Derived, VarFieldInfo &TypeInfo,
				 uint64_t currentOffset,
				 std::unordered_set<DIDerivedType *> &visited)
{
	if (visited.find(Derived) != visited.end()) {
		auto it = std::find_if(
			TypeInfo.FieldSizesAndOffsets.begin(), TypeInfo.FieldSizesAndOffsets.end(),
			[currentOffset, Derived](const std::pair<uint64_t, uint64_t> &entry) {
				return entry.second == currentOffset &&
				       entry.first == Derived->getSizeInBits();
			});
		if (it != TypeInfo.FieldSizesAndOffsets.end()) {
			errs() << " \n Already visited type ";
			Derived->dump();
			errs() << "\n Current offset: " << currentOffset;
			return;
		}
	}
	visited.insert(Derived);

	if (Derived->getTag() == llvm::dwarf::DW_TAG_member) {

		uint64_t offset = currentOffset + (Derived->getOffsetInBits() / 8);

		auto *newBase = Derived->getBaseType();
		while (newBase) {
			if (auto *RealBase = dyn_cast<DIBasicType>(newBase)) {
				uint64_t size = Derived->getSizeInBits() / 8;
				errs() << "\n Identified base type: ";
				RealBase->dump();
				errs() << "\n pushed back with offset: " << offset;
				TypeInfo.FieldSizesAndOffsets.push_back(
					std::make_pair(size, offset));
				break;
			} else if (auto *DerivedBase = dyn_cast<DIDerivedType>(newBase)) {
				collectFieldSizesAndOffsets(DerivedBase, TypeInfo, offset, visited);
				break;
			} else if (auto *CompositeBase = dyn_cast<DICompositeType>(newBase)) {
				if (DINodeArray elements = CompositeBase->getElements()) {
					for (auto element : elements) {
						if (DIDerivedType *Derived2 =
							    dyn_cast<DIDerivedType>(element)) {
							collectFieldSizesAndOffsets(
								Derived2, TypeInfo, offset,
								visited);
						}
					}
				}
				break;
			} else {
				errs() << " \n Could not identify base type";
				break;
			}
		}
	} else if (Derived->getTag() == llvm::dwarf::DW_TAG_pointer_type) {
		// uint64_t offset = currentOffset + 8;
		// TypeInfo.FieldSizesAndOffsets.push_back(std::make_pair(8, currentOffset + 8));

		errs() << "\n pointer type: ";
		Derived->dump();
		errs() << "\n Current offset: " << currentOffset;
	} else {
		errs() << " \n Could not identify tag type";
		Derived->dump();
	}
}

VarFieldInfo collectDbgDeclareInfo(DbgDeclareInst *DebugDeclareI)
{
	VarFieldInfo TypeInfo;
	DILocalVariable *Var = DebugDeclareI->getVariable();
	TypeInfo.VariableName = Var->getName().str();
	if (DICompositeType *Type = dyn_cast<DICompositeType>(Var->getType())) {
		if (DINodeArray elements = Type->getElements()) {
			std::unordered_set<DIDerivedType *> visited;
			for (auto element : elements) {
				if (DIDerivedType *Derived = dyn_cast<DIDerivedType>(element)) {
					collectFieldSizesAndOffsets(Derived, TypeInfo, 0, visited);
				}
			}
		}
	}
	return TypeInfo;
}

auto promoteMemcpy(MemCpyInst *MI, SmallVector<llvm::MemIntrinsic *> &promoted) -> bool
{
	ConstantInt *constLength = llvm::dyn_cast<llvm::ConstantInt>(MI->getLength());
	if (!constLength) {
		WARN_ONCE("memintr-length",
			  "Cannot promote non-constant-length mem intrinsic! Skipping...\n");
		return false;
	}

	uint64_t lengthValue = constLength->getZExtValue();
	IRBuilder<> builder(MI);
	LLVMContext &ctx = MI->getContext();
	Type *int8Type = Type::getInt8Ty(ctx);
	Value *destPtr = builder.CreateBitCast(MI->getRawDest(), int8Type->getPointerTo());
	Value *srcPtr = builder.CreateBitCast(MI->getRawSource(), int8Type->getPointerTo());

	if (lengthValue % 8 == 0) {
		Type *I64Type = Type::getInt64Ty(ctx);
		int size_in_i64s = static_cast<int>(lengthValue) / 8;
		for (uint64_t i = 0; i < size_in_i64s; i++) {
			Value *index = ConstantInt::get(builder.getInt64Ty(), i * 8);
			Value *srcGEP = builder.CreateGEP(int8Type, srcPtr, index, "src_gep");
			Value *destGEP = builder.CreateGEP(int8Type, destPtr, index, "dest_gep");
			Value *loadedVal = builder.CreateLoad(I64Type, srcGEP, "load_i64");
			builder.CreateStore(loadedVal, destGEP);
		}
	} else if (lengthValue % 4 == 0) {
		Type *I32Type = Type::getInt32Ty(ctx);
		Value *loadedVal = builder.CreateLoad(I32Type, srcPtr, "load_i32");
		builder.CreateStore(loadedVal, destPtr);
	} else if (lengthValue % 2 == 0) {
		Type *I16Type = Type::getInt16Ty(ctx);
		Value *loadedVal = builder.CreateLoad(I16Type, srcPtr, "load_i16");
		builder.CreateStore(loadedVal, destPtr);
	} else {
		for (uint64_t i = 0; i < lengthValue; ++i) {
			Value *index = ConstantInt::get(builder.getInt64Ty(), i);
			Value *srcGEP = builder.CreateGEP(int8Type, srcPtr, index, "src_gep");
			Value *destGEP = builder.CreateGEP(int8Type, destPtr, index, "dest_gep");
			Value *loadedVal = builder.CreateLoad(int8Type, srcGEP, "load_byte");
			builder.CreateStore(loadedVal, destGEP);
		}
	}

	promoted.push_back(MI);
	return true;
}

auto PromoteUndefMemset(MemSetInst *MI, SmallVector<llvm::MemIntrinsic *> &promoted)
{
	Type *OpaquePtrTy = PointerType::get(MI->getContext(), 0);
	Type *I8Ty = Type::getInt8Ty(MI->getContext());
	Value *UndefI64 = UndefValue::get(OpaquePtrTy);

	auto *MemsetDest = MI->getDest();

	Value *Len = MI->getLength();
	if (auto *CI = dyn_cast<ConstantInt>(Len)) {
		uint64_t lengthVal = CI->getZExtValue(); // or getSExtValue()
							 // Use lengthVal

		IRBuilder<> builder(MI);

		for (uint64_t i = 0; i < lengthVal; i = i + 8) {
			// Indices for GEP: [0, i]
			Value *Indices[1] = {
				ConstantInt::get(Type::getInt64Ty(MI->getContext()), i)};

			// Create a pointer to the i-th element
			// GEP type is ArrTy, the pointer operand is the alloca
			Value *ElemPtr = builder.CreateGEP(I8Ty,       // Pointee type
							   MemsetDest, // Base pointer
							   Indices,    // GEP indices
							   "elemPtr");

			builder.CreateStore(UndefI64, ElemPtr);
		}

		return true;
	} else {
		errs() << " \n Cannot promote memset with non-constant length: ";
		MI->dump();
		abort();
		return false;
	}
}

auto PromoteMemcpy::run(Function &F, FunctionAnalysisManager &FAM) -> PreservedAnalyses
{
	auto modified = false;
	SmallVector<llvm::MemIntrinsic *> promoted;
	int found = 0;

	for (auto &I : instructions(F)) {
		if (auto *MI = dyn_cast<MemCpyInst>(&I)) {
			ConstantInt *constLength =
				llvm::dyn_cast<llvm::ConstantInt>(MI->getLength());
			if (!constLength) {
				WARN_ONCE("memintr-length", "Cannot promote non-constant-length "
							    "mem intrinsic! Skipping...\n");
				continue;
			}

			auto DbgDeclares = findDbgDeclares(MI->getSource());
			if (DbgDeclares.size() == 1) {
				VarFieldInfo TypeInfo = collectDbgDeclareInfo(DbgDeclares[0]);
				if (!TypeInfo.FieldSizesAndOffsets.empty()) {
					found++;
					convertMemcpyWithDebugInfo(MI, TypeInfo, promoted);
				} else {
					modified |= promoteMemcpy(MI, promoted);
				}
			} else {
				modified |= promoteMemcpy(MI, promoted);
			}
		} else if (auto *MS = dyn_cast<MemSetInst>(&I)) {
			if (auto *UV = dyn_cast<UndefValue>(MS->getArgOperand(1))) {
				errs() << " \n EGG. EGG. EGG. EGG. EGG. EGG. EGG. EGG. EGG. EGG. ";
				modified |= PromoteUndefMemset(MS, promoted);
			} else {
				modified |= promoteI64Memset(MS, promoted);
			}
		}
	}
	removePromoted(promoted);
	return modified ? PreservedAnalyses::none() : PreservedAnalyses::all();
}
