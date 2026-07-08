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
 */

#include "RustTestDiscovery.hpp"

#include <llvm/Demangle/Demangle.h>
#include <llvm/IR/Constants.h>
#include <llvm/IR/Function.h>
#include <llvm/IR/Instructions.h>
#include <llvm/ADT/SmallVector.h>
#include <llvm/IR/Module.h>
#include <llvm/Support/Regex.h>

#include <algorithm>
#include <set>

using namespace llvm;

/* Extract the test function from a single TestDescAndFn struct initializer.
 * The struct contains data fields (byte arrays, string pointers) and exactly
 * one FnOnce::call_once function pointer — the test entry. We find it by
 * scanning the struct's top-level fields for a Function reference. */
static auto extractTestFnFromDescriptor(const Constant *descriptor) -> const Function *
{
	auto *gv = dyn_cast<GlobalVariable>(descriptor);
	if (!gv || !gv->hasInitializer())
		return nullptr;

	auto *init = gv->getInitializer();
	auto *st = dyn_cast<ConstantAggregate>(init);
	if (!st)
		return nullptr;

	for (unsigned i = 0; i < st->getNumOperands(); ++i) {
		auto *op = st->getOperand(i)->stripPointerCasts();
		if (auto *fn = dyn_cast<Function>(op))
			return fn;
	}
	return nullptr;
}

static auto findLangStartUserMain(const Module &mod) -> const Function *
{
	auto *mainFn = mod.getFunction("main");
	if (!mainFn || mainFn->isDeclaration())
		return nullptr;

	Regex langStartRe("^_ZN3std2rt10lang_start|^_RNvNtCs.*3std2rt10lang_start");
	for (auto &BB : *mainFn) {
		for (auto &I : BB) {
			auto *call = dyn_cast<CallInst>(&I);
			if (!call)
				continue;
			auto *callee = call->getCalledFunction();
			if (!callee || !langStartRe.match(callee->getName()))
				continue;
			if (call->arg_size() < 1)
				continue;
			return dyn_cast<Function>(call->getArgOperand(0));
		}
	}
	return nullptr;
}

static auto findTestMainStaticArg(const Function *userMain) -> const GlobalVariable *
{
	if (!userMain || userMain->isDeclaration())
		return nullptr;

	for (auto &BB : *userMain) {
		for (auto &I : BB) {
			auto *call = dyn_cast<CallInst>(&I);
			if (!call)
				continue;
			auto *callee = call->getCalledFunction();
			if (!callee)
				continue;
			auto demangled = llvm::demangle(callee->getName().str());
			if (demangled.find("test_main_static") == std::string::npos)
				continue;
			if (call->arg_size() < 1)
				continue;
			auto *arg = call->getArgOperand(0)->stripPointerCasts();
			if (auto *gv = dyn_cast<GlobalVariable>(arg))
				return gv;
		}
	}
	return nullptr;
}

/* Follow the call_once → closure → test chain structurally.
 * At each level, find defined non-intrinsic callees. If there is
 * exactly one, follow it. If there are multiple, pick the one
 * that takes zero arguments (the test fn, not assert_test_result).
 * Stop when the function has zero arguments itself (the actual test). */
static auto resolveTestFunction(const Function *fn, unsigned depth = 0) -> const Function *
{
	if (!fn || fn->isDeclaration() || depth > 5)
		return fn;
	if (fn->arg_size() == 0)
		return fn;

	SmallVector<const Function *, 4> callees;
	for (auto &BB : *fn) {
		for (auto &I : BB) {
			auto *call = dyn_cast<CallInst>(&I);
			if (!call)
				continue;
			auto *callee = call->getCalledFunction();
			if (!callee || callee->isDeclaration() || callee->isIntrinsic())
				continue;
			callees.push_back(callee);
		}
	}

	if (callees.empty())
		return fn;

	/* Prefer the zero-argument callee (the actual test function) */
	for (auto *c : callees) {
		if (c->arg_size() == 0)
			return resolveTestFunction(c, depth + 1);
	}

	/* Otherwise follow the single callee if there is one */
	if (callees.size() == 1)
		return resolveTestFunction(callees[0], depth + 1);

	return fn;
}

static auto discoverViaTestHarness(const Module &mod) -> std::vector<RustTestInfo>
{
	std::vector<RustTestInfo> results;

	auto *userMain = findLangStartUserMain(mod);
	auto *testsGV = findTestMainStaticArg(userMain);
	if (!testsGV || !testsGV->hasInitializer())
		return results;

	std::set<std::string> seen;
	auto addEntry = [&](const Constant *entry) {
		auto *fn = extractTestFnFromDescriptor(entry);
		if (!fn || fn->isDeclaration())
			return;
		auto *resolved = resolveTestFunction(fn);
		if (!resolved || resolved->isDeclaration())
			return;
		auto name = resolved->getName().str();
		if (!seen.insert(name).second)
			return;
		results.push_back({name, llvm::demangle(name)});
	};

	/* The test array is normally an aggregate of pointers to TestDescAndFn
	 * globals — walk each entry and extract its test function pointer. With
	 * exactly one #[test], rustc collapses the [1 x &TestDescAndFn] array to a
	 * bare pointer to the single descriptor, so handle that shape too. */
	auto *arr = testsGV->getInitializer();
	if (auto *arrStruct = dyn_cast<ConstantAggregate>(arr)) {
		for (unsigned i = 0; i < arrStruct->getNumOperands(); ++i)
			addEntry(arrStruct->getOperand(i)->stripPointerCasts());
	} else {
		addEntry(arr->stripPointerCasts());
	}

	std::sort(results.begin(), results.end(),
		  [](const RustTestInfo &a, const RustTestInfo &b) {
			  return a.demangledName < b.demangledName;
		  });
	return results;
}

static auto applyFilter(std::vector<RustTestInfo> tests, const std::string &filter)
	-> std::vector<RustTestInfo>
{
	if (filter.empty())
		return tests;

	std::vector<RustTestInfo> filtered;
	for (auto &t : tests) {
		if (t.demangledName.find(filter) != std::string::npos)
			filtered.push_back(std::move(t));
	}
	return filtered;
}

auto discoverRustTests(const Module &mod, const std::string &filter) -> std::vector<RustTestInfo>
{
	auto tests = discoverViaTestHarness(mod);
	return applyFilter(std::move(tests), filter);
}
