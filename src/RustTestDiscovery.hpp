#ifndef GENMC_RUST_TEST_DISCOVERY_HPP
#define GENMC_RUST_TEST_DISCOVERY_HPP

#include <llvm/IR/Module.h>
#include <string>
#include <vector>

struct RustTestInfo {
	std::string mangledName;
	std::string demangledName;
};

std::vector<RustTestInfo> discoverRustTests(const llvm::Module &mod,
					    const std::string &filter = "");

#endif
