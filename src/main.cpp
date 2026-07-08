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

#include "Config.hpp"
#include "DriverFactory.hpp"
#include "Error.hpp"
#include "RustTestDiscovery.hpp"
#include "Static/LLVMModule.hpp"
#include "config.h"

#include <llvm/Demangle/Demangle.h>

#include <algorithm>
#include <chrono>
#include <cmath>
#include <cstdlib>
#include <fcntl.h>
#include <fstream>
#include <future>
#include <iostream>
#include <memory>
#include <numeric>
#include <set>
#include <signal.h>
#include <sys/wait.h>
#include <unistd.h>

auto getOutFilename(const std::shared_ptr<const Config> & /*conf*/) -> std::string
{
	static char filenameTemplate[] = "/tmp/__genmc.ll.XXXXXX";
	static bool createdFilename = false;

	if (!createdFilename) {
		close(mkstemp(filenameTemplate));
		createdFilename = true;
	}
	return {filenameTemplate};
}

auto buildCompilationArgs(const std::shared_ptr<const Config> &conf) -> std::string
{
	std::string args;

	args += " -fno-discard-value-names";
	args += " -Xclang";
	args += " -disable-O0-optnone";
	args += " -g"; /* Compile with -g to get debugging mdata */
	for (const auto &f : conf->cflags)
		args += " " + f;
	args += " -I" SRC_INCLUDE_DIR;
	args += " -I" INCLUDE_DIR;
	auto inodeFlag = " -D__CONFIG_GENMC_INODE_DATA_SIZE=" + std::to_string(conf->maxFileSize);
	args += " " + inodeFlag;
	args += " -S -emit-llvm";
	args += " -o " + getOutFilename(conf);
	args += " " + conf->inputFile;

	return args;
}

auto compileInput(const std::shared_ptr<const Config> &conf,
		  const std::unique_ptr<llvm::LLVMContext> &ctx,
		  std::unique_ptr<llvm::Module> &module) -> bool
{
	const auto *path = CLANGPATH;
	auto command = path + buildCompilationArgs(conf);
	if (std::system(command.c_str()) != 0)
		return false;

	module = LLVMModule::parseLLVMModule(getOutFilename(conf), ctx);
	return true;
}

void transformInput(const std::shared_ptr<Config> &conf, llvm::Module &module, ModuleInfo &modInfo)
{
	LLVMModule::transformLLVMModule(module, modInfo, conf);
	if (!conf->transformFile.empty())
		LLVMModule::printLLVMModule(module, conf->transformFile);
	if (!conf->transformBcFile.empty())
		LLVMModule::writeBitcodeModule(module, conf->transformBcFile);

	/* Perhaps override the MM under which verification will take place */
	if (conf->mmDetector && modInfo.determinedMM.has_value() &&
	    isStrongerThan(*modInfo.determinedMM, conf->model)) {
		conf->model = *modInfo.determinedMM;
		conf->isDepTrackingModel = (conf->model == ModelType::IMM);
		LOG(VerbosityLevel::Tip)
			<< "Automatically adjusting memory model to " << conf->model
			<< ". You can disable this behavior with -disable-mm-detector.\n";
	}
}

auto getElapsedSecs(const std::chrono::high_resolution_clock::time_point &begin) -> long double
{
	static constexpr long double secToMillFactor = 1e-3L;
	auto now = std::chrono::high_resolution_clock::now();
	return std::chrono::duration_cast<std::chrono::milliseconds>(now - begin).count() *
	       secToMillFactor;
}

void printEstimationResults(const std::shared_ptr<const Config> &conf,
			    const std::chrono::high_resolution_clock::time_point &begin,
			    const GenMCDriver::Result &res)
{
	PRINT(VerbosityLevel::Error) << res.message;
	PRINT(VerbosityLevel::Error)
		<< (res.status == VerificationError::VE_OK ? "*** Estimation complete.\n"
							   : "*** Estimation unsuccessful.\n");

	auto mean = std::llround(res.estimationMean);
	auto sd = std::llround(std::sqrt(res.estimationVariance));
	auto meanTimeSecs = getElapsedSecs(begin) / (res.explored + res.exploredBlocked);
	PRINT(VerbosityLevel::Error)
		<< "Total executions estimate: " << mean << " (+- " << sd << ")\n"
		<< "Time to completion estimate: " << llvm::format("%.2Lf", meanTimeSecs * mean)
		<< "s\n";
	GENMC_DEBUG(if (conf->printEstimationStats) PRINT(VerbosityLevel::Error)
			    << "Estimation moot: " << res.exploredMoot << "\n"
			    << "Estimation blocked: " << res.exploredBlocked << "\n"
			    << "Estimation complete: " << res.explored << "\n";);
}

void printVerificationResults(const std::shared_ptr<const Config> &conf,
			      const std::chrono::high_resolution_clock::time_point &begin,
			      const GenMCDriver::Result &res)
{
	PRINT(VerbosityLevel::Error) << res.message;
	PRINT(VerbosityLevel::Error)
		<< (res.status == VerificationError::VE_OK
			    ? "*** Verification complete. No errors were detected.\n"
			    : "*** Verification unsuccessful.\n");

	PRINT(VerbosityLevel::Error) << "Number of complete executions explored: " << res.explored;
	GENMC_DEBUG(PRINT(VerbosityLevel::Error)
			    << ((conf->countDuplicateExecs)
					? " (" + std::to_string(res.duplicates) + " duplicates)"
					: ""););
	if (res.boundExceeding) {
		BUG_ON(conf->boundType == BoundType::round);
		PRINT(VerbosityLevel::Error)
			<< " (" + std::to_string(res.boundExceeding) + " exceeded bound)";
	}
	if (res.exploredBlocked != 0U) {
		PRINT(VerbosityLevel::Error)
			<< "\nNumber of blocked executions seen: " << res.exploredBlocked;
	}
	GENMC_DEBUG(
		if (conf->countMootExecs) {
			PRINT(VerbosityLevel::Error) << " (+ " << res.exploredMoot << " mooted)";
		};
		if (conf->boundsHistogram) {
			PRINT(VerbosityLevel::Error) << "\nBounds histogram:";
			auto executions = 0u;
			for (auto i = 0u; i < res.exploredBounds.size(); i++) {
				executions += res.exploredBounds[i];
				PRINT(VerbosityLevel::Error) << " " << executions;
			}
			if (!executions)
				PRINT(VerbosityLevel::Error) << " 0";
		});
	PRINT(VerbosityLevel::Error)
		<< "\nTotal wall-clock time: " << llvm::format("%.2Lf", getElapsedSecs(begin))
		<< "s\n";
}

auto main(int argc, char **argv) -> int
{
	auto begin = std::chrono::high_resolution_clock::now();
	auto conf = std::make_shared<Config>();

	conf->getConfigOptions(argc, argv);

	PRINT(VerbosityLevel::Error)
		<< PACKAGE_NAME " v" PACKAGE_VERSION << " (LLVM " LLVM_VERSION ")\n"
		<< "Copyright (C) 2024 MPI-SWS. All rights reserved.\n\n";

	auto ctx = std::make_unique<llvm::LLVMContext>(); // *dtor after module's*
	std::unique_ptr<llvm::Module> module;
	if (conf->inputFromBitcodeFile) {
		module = LLVMModule::parseLLVMModule(conf->inputFile, ctx);
	} else if (!compileInput(conf, ctx, module)) {
		return ECOMPILE;
	}
	PRINT(VerbosityLevel::Error) << "*** Compilation complete.\n";

	/* Discover Rust test functions if requested (before transforms) */
	if (conf->discoverRustTests) {
		auto tests = discoverRustTests(*module, conf->rustTestFilter);
		if (tests.empty()) {
			PRINT(VerbosityLevel::Error)
				<< "*** No Rust test functions discovered.\n";
			return 1;
		}

		PRINT(VerbosityLevel::Error)
			<< "*** Discovered " << tests.size() << " Rust test function(s).\n";

		if (conf->listRustTests) {
			for (const auto &t : tests) {
				PRINT(VerbosityLevel::Error)
					<< "  " << t.demangledName
					<< " [" << t.mangledName << "]\n";
			}
			return 0;
		}

		conf->programEntryFuns.clear();
		for (const auto &t : tests)
			conf->programEntryFuns.push_back(t.mangledName);
		conf->programEntryFun = conf->programEntryFuns[0];
	}

	/* Perform the necessary transformations */
	auto modInfo = std::make_unique<ModuleInfo>(*module);
	if (conf->skipTransforms) {
		LLVMModule::collectModuleInfo(*module, *modInfo);
		PRINT(VerbosityLevel::Error) << "*** Skipping transforms (pre-transformed input).\n";
	} else {
		transformInput(conf, *module, *modInfo);
		PRINT(VerbosityLevel::Error) << "*** Transformation complete.\n";
	}

	/* If transform-only mode, exit after transforms */
	if (conf->transformOnly) {
		PRINT(VerbosityLevel::Error) << "*** Transform-only mode: exiting without verification.\n";
		return 0;
	}

	/* Estimate the state space (only for single-entry mode) */
	if (conf->estimate && conf->programEntryFuns.size() == 1) {
		LOG(VerbosityLevel::Tip) << "Estimating state-space size. For better performance, "
					    "you can use --disable-estimation.\n";
		auto res = GenMCDriver::estimate(conf, module, modInfo);
		printEstimationResults(conf, begin, res);
		if (res.status != VerificationError::VE_OK)
			return EVERIFY;
	}

	/* Verify each entry function */
	int overallResult = 0;
	for (size_t i = 0; i < conf->programEntryFuns.size(); ++i) {
		const auto &entryFun = conf->programEntryFuns[i];
		conf->programEntryFun = entryFun;

		auto displayName = conf->discoverRustTests
					  ? llvm::demangle(entryFun)
					  : entryFun;

		if (conf->programEntryFuns.size() > 1) {
			PRINT(VerbosityLevel::Error)
				<< "=== Verifying: " << displayName << " ===\n";
		}

		/* Redirect output to per-test trace file if requested */
		int savedOut = -1, savedErr = -1;
		if (!conf->traceOutputDir.empty()) {
			std::string safeName = displayName;
			std::replace(safeName.begin(), safeName.end(), ':', '_');
			auto filename = conf->traceOutputDir + "/" + safeName + "_verification.txt";

			llvm::outs().flush();
			llvm::errs().flush();
			savedOut = dup(STDOUT_FILENO);
			savedErr = dup(STDERR_FILENO);
			int fd = open(filename.c_str(), O_WRONLY | O_CREAT | O_TRUNC, 0644);
			dup2(fd, STDOUT_FILENO);
			dup2(fd, STDERR_FILENO);
			close(fd);
		}

		auto testBegin = std::chrono::high_resolution_clock::now();
		GenMCDriver::Result res;
		bool isLast = (i + 1 == conf->programEntryFuns.size());
		bool timedOut = false;

		if (conf->perTestTimeout > 0) {
			/* Run verification with per-test timeout using fork */
			auto pid = fork();
			if (pid == 0) {
				/* Child has its own copy of module via fork */
				auto cloneCtx = std::make_unique<llvm::LLVMContext>();
				auto clonedMod = LLVMModule::cloneModule(module, cloneCtx);
				auto clonedMI = modInfo->clone(*clonedMod);
				res = GenMCDriver::verify(conf, std::move(clonedMod), std::move(clonedMI));
				printVerificationResults(conf, testBegin, res);
				llvm::outs().flush();
				llvm::errs().flush();
				_exit(res.status != VerificationError::VE_OK ? EVERIFY : 0);
			} else {
				/* Parent: wait with timeout */
				int status = 0;
				auto deadline = std::chrono::steady_clock::now() +
						std::chrono::seconds(conf->perTestTimeout);
				pid_t waited = 0;
				while (std::chrono::steady_clock::now() < deadline) {
					waited = waitpid(pid, &status, WNOHANG);
					if (waited != 0)
						break;
					usleep(100000); /* 100ms */
				}
				if (waited == 0) {
					kill(pid, SIGKILL);
					waitpid(pid, &status, 0);
					timedOut = true;
					PRINT(VerbosityLevel::Error) << "TIMEOUT\n";
				} else if (WIFEXITED(status) && WEXITSTATUS(status) != 0) {
					overallResult = EVERIFY;
				}
				if (isLast) {
					module.reset();
					modInfo.reset();
				}
			}
		} else {
			if (isLast) {
				res = GenMCDriver::verify(conf, std::move(module), std::move(modInfo));
			} else {
				auto cloneCtx = std::make_unique<llvm::LLVMContext>();
				auto clonedMod = LLVMModule::cloneModule(module, cloneCtx);
				auto clonedMI = modInfo->clone(*clonedMod);
				res = GenMCDriver::verify(conf, std::move(clonedMod), std::move(clonedMI));
			}
			printVerificationResults(conf, testBegin, res);
			if (res.status != VerificationError::VE_OK)
				overallResult = EVERIFY;
		}

		/* Restore output */
		if (savedOut != -1) {
			llvm::outs().flush();
			llvm::errs().flush();
			fflush(stdout);
			fflush(stderr);
			dup2(savedOut, STDOUT_FILENO);
			dup2(savedErr, STDERR_FILENO);
			close(savedOut);
			close(savedErr);
		}
	}

	/* TODO: Check globalContext.destroy() and llvm::shutdown() */
	return overallResult;
}
