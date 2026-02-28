/*
 * GenMC -- Generic Model Checking.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, you can access it online at
 * http://www.gnu.org/licenses/gpl-3.0.html.
 *
 * Author: Michalis Kokologiannakis <michalis@mpi-sws.org>
 */

#include "IntrinsicLoweringPass.hpp"

#include <llvm/ADT/Twine.h>
#include <llvm/CodeGen/IntrinsicLowering.h>
#include <llvm/IR/BasicBlock.h>
#include <llvm/IR/Function.h>
#include <llvm/IR/IRBuilder.h>
#include <llvm/IR/InstrTypes.h>
#include <llvm/IR/Instruction.h>
#include <llvm/IR/Instructions.h>
#include <llvm/IR/IntrinsicInst.h>
#include <llvm/IR/Module.h>
#include <llvm/IR/Type.h>
#include <llvm/Transforms/Utils/BasicBlockUtils.h>

using namespace llvm;

auto runOnBasicBlock(BasicBlock &BB, IntrinsicLowering *IL) -> bool
{
	int i = 0;
	Value *lhs = NULL;
	Value *rhs = NULL;
	auto &M = *BB.getParent()->getParent();
	auto modified = false;
	for (auto it = BB.begin(); it != BB.end();) {
		auto *I = llvm::dyn_cast<IntrinsicInst>(&*it);
		/* Iterator is incremented in order for it not to be invalidated */
		++it;
		/* If the instruction is not an intrinsic call, skip it */
		if (!I)
			continue;
		switch (I->getIntrinsicID()) {
#if LLVM_VERSION_MAJOR >= 16
		/* In case thread-local variables are not accessed directly, make them */
		case llvm::Intrinsic::threadlocal_address:
			I->replaceAllUsesWith(I->getOperand(0));
			break;
#endif
		case llvm::Intrinsic::vastart:
		case llvm::Intrinsic::vaend:
		case llvm::Intrinsic::vacopy:
		case llvm::Intrinsic::is_constant:
		case llvm::Intrinsic::powi:
		case llvm::Intrinsic::fptoui_sat:
		case llvm::Intrinsic::fabs:
		case llvm::Intrinsic::fptosi_sat:
		case llvm::Intrinsic::minnum:
		case llvm::Intrinsic::maxnum:
		case llvm::Intrinsic::abs:
		case Intrinsic::fma:
		case Intrinsic::bitreverse:
			break;

		// Lower smax/smin/umax/umin to icmp + select
		case Intrinsic::smax:
		case Intrinsic::smin:
		case Intrinsic::umax:
		case Intrinsic::umin: {
			IRBuilder<> builder(I->getParent(), I->getIterator());

			Value *op1 = I->getArgOperand(0);
			Value *op2 = I->getArgOperand(1);

			Value *cmp = nullptr;
			switch (I->getIntrinsicID()) {
			case Intrinsic::smax:
				cmp = builder.CreateICmpSGT(op1, op2);
				break;
			case Intrinsic::smin:
				cmp = builder.CreateICmpSLT(op1, op2);
				break;
			case Intrinsic::umax:
				cmp = builder.CreateICmpUGT(op1, op2);
				break;
			case Intrinsic::umin:
				cmp = builder.CreateICmpULT(op1, op2);
				break;
			default:
				llvm_unreachable("Unexpected intrinsic");
			}

			Value *result = builder.CreateSelect(cmp, op1, op2);
			I->replaceAllUsesWith(result);
			I->eraseFromParent();
			modified = true;
			break;
		}

		// Attempt to deal with overflow intrinsics taken from KLEE:
		// https://github.com/klee/klee/blob/master/lib/Module/IntrinsicCleaner.cpp
		case Intrinsic::sadd_with_overflow:
		case Intrinsic::ssub_with_overflow:
		case Intrinsic::smul_with_overflow:
		case Intrinsic::uadd_with_overflow:
		case Intrinsic::usub_with_overflow:
		case Intrinsic::umul_with_overflow: {
			IRBuilder<> builder(I->getParent(), I->getIterator());

			Value *op1 = I->getArgOperand(0);
			Value *op2 = I->getArgOperand(1);

			Value *result = 0;
			Value *result_ext = 0;
			Value *overflow = 0;

			unsigned int bw = op1->getType()->getPrimitiveSizeInBits();
			unsigned int bw2 = op1->getType()->getPrimitiveSizeInBits() * 2;

			if ((I->getIntrinsicID() == Intrinsic::uadd_with_overflow) ||
			    (I->getIntrinsicID() == Intrinsic::usub_with_overflow) ||
			    (I->getIntrinsicID() == Intrinsic::umul_with_overflow)) {

				Value *op1ext = builder.CreateZExt(
					op1, IntegerType::get(M.getContext(), bw2));
				Value *op2ext = builder.CreateZExt(
					op2, IntegerType::get(M.getContext(), bw2));
				Value *int_max_s =
					ConstantInt::get(op1->getType(), APInt::getMaxValue(bw));
				Value *int_max = builder.CreateZExt(
					int_max_s, IntegerType::get(M.getContext(), bw2));

				if (I->getIntrinsicID() == Intrinsic::uadd_with_overflow) {
					result_ext = builder.CreateAdd(op1ext, op2ext);
				} else if (I->getIntrinsicID() == Intrinsic::usub_with_overflow) {
					result_ext = builder.CreateSub(op1ext, op2ext);
				} else if (I->getIntrinsicID() == Intrinsic::umul_with_overflow) {
					result_ext = builder.CreateMul(op1ext, op2ext);
				}
				overflow = builder.CreateICmpUGT(result_ext, int_max);

			} else if ((I->getIntrinsicID() == Intrinsic::sadd_with_overflow) ||
				   (I->getIntrinsicID() == Intrinsic::ssub_with_overflow) ||
				   (I->getIntrinsicID() == Intrinsic::smul_with_overflow)) {

				Value *op1ext = builder.CreateSExt(
					op1, IntegerType::get(M.getContext(), bw2));
				Value *op2ext = builder.CreateSExt(
					op2, IntegerType::get(M.getContext(), bw2));
				Value *int_max_s = ConstantInt::get(op1->getType(),
								    APInt::getSignedMaxValue(bw));
				Value *int_min_s = ConstantInt::get(op1->getType(),
								    APInt::getSignedMinValue(bw));
				Value *int_max = builder.CreateSExt(
					int_max_s, IntegerType::get(M.getContext(), bw2));
				Value *int_min = builder.CreateSExt(
					int_min_s, IntegerType::get(M.getContext(), bw2));

				if (I->getIntrinsicID() == Intrinsic::sadd_with_overflow) {
					result_ext = builder.CreateAdd(op1ext, op2ext);
				} else if (I->getIntrinsicID() == Intrinsic::ssub_with_overflow) {
					result_ext = builder.CreateSub(op1ext, op2ext);
				} else if (I->getIntrinsicID() == Intrinsic::smul_with_overflow) {
					result_ext = builder.CreateMul(op1ext, op2ext);
				}
				overflow = builder.CreateOr(
					builder.CreateICmpSGT(result_ext, int_max),
					builder.CreateICmpSLT(result_ext, int_min));
			}

			// This trunc cound be replaced by a more general trunc replacement
			// that allows to detect also undefined behavior in assignments or
			// overflow in operation with integers whose dimension is smaller than
			// int's dimension, e.g.
			//     uint8_t = uint8_t + uint8_t;
			// if one desires the wrapping should write
			//     uint8_t = (uint8_t + uint8_t) & 0xFF;
			// before this, must check if it has side effects on other operations
			result = builder.CreateTrunc(result_ext, op1->getType());
			Value *resultStruct =
				builder.CreateInsertValue(UndefValue::get(I->getType()), result, 0);
			resultStruct = builder.CreateInsertValue(resultStruct, overflow, 1);

			I->replaceAllUsesWith(resultStruct);
			I->eraseFromParent();
			modified = true;
			break;
		}

		// Code taken from KLEE IntrinsicCleaner.cpp
		case Intrinsic::sadd_sat:
		case Intrinsic::ssub_sat:
		case Intrinsic::uadd_sat:
		case Intrinsic::usub_sat: {
			IRBuilder<> builder(I);

			Value *op1 = I->getArgOperand(0);
			Value *op2 = I->getArgOperand(1);

			unsigned int bw = op1->getType()->getPrimitiveSizeInBits();
			assert(bw == op2->getType()->getPrimitiveSizeInBits());

			Value *overflow = nullptr;
			Value *result = nullptr;
			Value *saturated = nullptr;
			switch (I->getIntrinsicID()) {
			case Intrinsic::usub_sat:
				result = builder.CreateSub(op1, op2);
				overflow = builder.CreateICmpULT(op1, op2); // a < b  =>  a - b < 0
				saturated = ConstantInt::get(M.getContext(), APInt(bw, 0));
				break;
			case Intrinsic::uadd_sat:
				result = builder.CreateAdd(op1, op2);
				overflow = builder.CreateICmpULT(result, op1); // a + b < a
				saturated =
					ConstantInt::get(M.getContext(), APInt::getMaxValue(bw));
				break;
			case Intrinsic::ssub_sat:
			case Intrinsic::sadd_sat: {
				if (I->getIntrinsicID() == Intrinsic::ssub_sat) {
					result = builder.CreateSub(op1, op2);
				} else {
					result = builder.CreateAdd(op1, op2);
				}
				ConstantInt *zero = ConstantInt::get(M.getContext(), APInt(bw, 0));
				ConstantInt *smin = ConstantInt::get(M.getContext(),
								     APInt::getSignedMinValue(bw));
				ConstantInt *smax = ConstantInt::get(M.getContext(),
								     APInt::getSignedMaxValue(bw));

				Value *sign1 = builder.CreateICmpSLT(op1, zero);
				Value *sign2 = builder.CreateICmpSLT(op2, zero);
				Value *signR = builder.CreateICmpSLT(result, zero);

				if (I->getIntrinsicID() == Intrinsic::ssub_sat) {
					saturated = builder.CreateSelect(sign2, smax, smin);
				} else {
					saturated = builder.CreateSelect(sign2, smin, smax);
				}

				// The sign of the result differs from the sign of the first operand
				overflow = builder.CreateXor(sign1, signR);
				if (I->getIntrinsicID() == Intrinsic::ssub_sat) {
					// AND the signs of the operands differ
					overflow = builder.CreateAnd(
						overflow, builder.CreateXor(sign1, sign2));
				} else {
					// AND the signs of the operands are the same
					overflow = builder.CreateAnd(
						overflow,
						builder.CreateNot(builder.CreateXor(sign1, sign2)));
				}
				break;
			}
			default:;
			}

			result = builder.CreateSelect(overflow, saturated, result);
			I->replaceAllUsesWith(result);
			I->eraseFromParent();
			modified = true;
			break;
		}

		case llvm::Intrinsic::dbg_value:
		case llvm::Intrinsic::dbg_declare:
			/* Remove useless calls to @llvm.debug.* */
			I->eraseFromParent();
			modified = true;
			break;
		case llvm::Intrinsic::trap: {
			// Intrinsic instruction "llvm.trap" found. Directly lower it to
			// a call of the abort() function.
			auto C = M.getOrInsertFunction("abort", Type::getVoidTy(M.getContext()));
			if (auto *F = dyn_cast<Function>(C.getCallee())) {
				F->setDoesNotReturn();
				F->setDoesNotThrow();
			}

			llvm::IRBuilder<> Builder(I);
			Builder.CreateCall(C);
			Builder.CreateUnreachable();

			it = I->eraseFromParent();

			// check if the instruction after the one we just replaced is not the
			// end of the basic block and if it is not (i.e. it is a valid
			// instruction), delete it and all remaining because the cleaner just
			// introduced a terminating instruction (unreachable) otherwise llvm will
			// assert in Verifier::visitTerminatorInstr
			while (it != BB.end()) { // i was already incremented above.
				it = it->eraseFromParent();
			}

			modified = true;
			break;
		}

		case Intrinsic::fshr: {
			IRBuilder<> builder(I->getParent(), I->getIterator());

			Value *op1 = I->getArgOperand(0); // a
			Value *op2 = I->getArgOperand(1); // b
			Value *op3 = I->getArgOperand(2); // shift amount

			unsigned int bw = op1->getType()->getPrimitiveSizeInBits();
			Value *bitWidthVal = ConstantInt::get(op1->getType(), bw);

			Value *lshr = builder.CreateLShr(op1, op3);
			Value *invShift = builder.CreateSub(bitWidthVal, op3);
			Value *shl = builder.CreateShl(op2, invShift);
			Value *result = builder.CreateOr(lshr, shl);

			I->replaceAllUsesWith(result);
			I->eraseFromParent();
			modified = true;
			break;
		}

		case Intrinsic::fshl: {
			IRBuilder<> builder(I->getParent(), I->getIterator());

			Value *op1 = I->getArgOperand(0); // a
			Value *op2 = I->getArgOperand(1); // b
			Value *op3 = I->getArgOperand(2); // shift amount

			unsigned int bw = op1->getType()->getPrimitiveSizeInBits();
			Value *bitWidthVal = ConstantInt::get(op1->getType(), bw);

			Value *shl = builder.CreateShl(op1, op3);
			Value *invShift = builder.CreateSub(bitWidthVal, op3);
			Value *lshr = builder.CreateLShr(op2, invShift);
			Value *result = builder.CreateOr(shl, lshr);

			I->replaceAllUsesWith(result);
			I->eraseFromParent();
			modified = true;
			break;
		}

		case Intrinsic::bswap: {
			if (auto *CI = dyn_cast<CallInst>(I)) {
				if (Value *V = CI->getArgOperand(0)) {
					unsigned BitSize = V->getType()->getScalarSizeInBits();

					if (BitSize == 16 || BitSize == 32 || BitSize == 64) {
						IL->LowerIntrinsicCall(I);
						modified = true;
						break;
					} else {
						// errs() << "Unhandled type size of value to
						// byteswap";
						break;
					}
				} else {
					break;
				}
			} else {
				break;
			}
		}

		case Intrinsic::load_relative: {
			/* Lower llvm.load.relative.iN(ptr base, iN offset) to:
			 *   entry_ptr = GEP i8, base, offset
			 *   rel_val   = load i32, entry_ptr
			 *   result    = GEP i8, entry_ptr, rel_val
			 * Relative vtable entries store offsets relative to the
			 * entry address (standard ELF relative relocation
			 * convention), so the final pointer is entry_ptr + rel_val,
			 * matching LLVM's own SelectionDAGBuilder lowering. */
			IRBuilder<> builder(I->getParent(), I->getIterator());

			Value *base = I->getArgOperand(0);
			Value *offset = I->getArgOperand(1);

			/* GEP to the table entry */
			Value *entryPtr = builder.CreateGEP(
				Type::getInt8Ty(M.getContext()), base, offset, "rel.entry");

			/* Load the relative offset (i32) */
			Value *relVal = builder.CreateLoad(
				Type::getInt32Ty(M.getContext()), entryPtr, "rel.val");

			/* Compute final pointer: entry_ptr + relative_offset */
			Value *result = builder.CreateGEP(
				Type::getInt8Ty(M.getContext()), entryPtr, relVal, "rel.result");

			I->replaceAllUsesWith(result);
			I->eraseFromParent();
			modified = true;
			break;
		}

		case Intrinsic::addressofreturnaddress:
		case Intrinsic::annotation:
		case Intrinsic::assume:
		case Intrinsic::ceil:
		case Intrinsic::copysign:
		case Intrinsic::cos:
		case Intrinsic::ctlz:
		case Intrinsic::ctpop:
		case Intrinsic::cttz:
		case Intrinsic::dbg_label:
		case Intrinsic::eh_typeid_for:
		case Intrinsic::exp2:
		case Intrinsic::exp:
		case Intrinsic::expect:
		case Intrinsic::experimental_noalias_scope_decl:
		case Intrinsic::floor:
		case Intrinsic::get_rounding:
		case Intrinsic::frameaddress:
		case Intrinsic::get_dynamic_area_offset:
		case Intrinsic::invariant_end:
		case Intrinsic::invariant_start:
		case Intrinsic::lifetime_end:
		case Intrinsic::lifetime_start:
		case Intrinsic::log10:
		case Intrinsic::log2:
		case Intrinsic::log:
		case Intrinsic::memcpy:
		case Intrinsic::memmove:
		case Intrinsic::memset:
		case Intrinsic::not_intrinsic:
		case Intrinsic::pcmarker:
		case Intrinsic::pow:
		case Intrinsic::prefetch:
		case Intrinsic::ptr_annotation:
		case Intrinsic::readcyclecounter:
		case Intrinsic::returnaddress:
		case Intrinsic::round:
		case Intrinsic::roundeven:
		case Intrinsic::sin:
		case Intrinsic::sqrt:
		case Intrinsic::stackrestore:
		case Intrinsic::stacksave:
		case Intrinsic::trunc:
		case Intrinsic::var_annotation:
			IL->LowerIntrinsicCall(I);
			modified = true;
			break;
		default:
			const Function *Callee = I->getCalledFunction();
			llvm::StringRef name = Callee->getName();
			errs() << "Unsupported intrinsic: " << name.data();
			break;
		}
	}
	return modified;
}

auto IntrinsicLoweringPass::run(Function &F, FunctionAnalysisManager &FAM) -> PreservedAnalyses
{
	auto ILUP = std::make_unique<IntrinsicLowering>(F.getParent()->getDataLayout());

	/* Scan through the instructions and lower intrinsic calls */
	auto modified = false;
	for (auto &BB : F)
		modified |= runOnBasicBlock(BB, &*ILUP);

	return modified ? PreservedAnalyses::none() : PreservedAnalyses::all();
}
