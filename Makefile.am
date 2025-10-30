# where to install the headers on the system
genmcdir = $(pkgincludedir)

# the list of header files that belong to the library
nobase_genmc_HEADERS = \
  include/assert.h \
  include/atomic \
  include/bits/stat.h \
  include/cassert \
  include/errno.h \
  include/fcntl.h \
  include/genmc.h \
  include/genmc_internal.h \
  include/lkmm.h \
  include/pthread.h \
  include/stdatomic.h \
  include/stdio.h \
  include/stdlib.h \
  include/sys/stat.h \
  include/sys/types.h \
  include/thread \
  include/threads.h \
  include/unistd.h

noinst_LIBRARIES = libgenmc.a
libgenmc_a_SOURCES = \
  src/AccessFilter.cpp src/AccessFilter.hpp \
  src/ASize.cpp src/ASize.hpp \
  src/Static/Transforms/BisimilarityCheckerPass.cpp src/Static/Transforms/BisimilarityCheckerPass.hpp \
  src/Bitmask.hpp \
  src/BoundDecider.cpp src/BoundDecider.hpp \
  src/Static/Transforms/CallInfoCollectionPass.cpp src/Static/Transforms/CallInfoCollectionPass.hpp \
  src/Static/Transforms/CodeCondenserPass.cpp src/Static/Transforms/CodeCondenserPass.hpp \
  src/ContextBoundDecider.cpp src/ContextBoundDecider.hpp \
  src/Calculator.hpp \
  src/CallInstWrapper.hpp \
  src/CombinatoricUtils.hpp \
  src/Static/Transforms/ConfirmationAnnotationPass.cpp src/Static/Transforms/ConfirmationAnnotationPass.hpp \
  src/Config.cpp src/Config.hpp \
  src/Static/Transforms/DeclareInternalsPass.cpp src/Static/Transforms/DeclareInternalsPass.hpp \
  src/Static/Transforms/DefineLibcFunsPass.cpp src/Static/Transforms/DefineLibcFunsPass.hpp \
  src/DepExecutionGraph.cpp src/DepExecutionGraph.hpp \
  src/DepInfo.cpp src/DepInfo.hpp \
  src/DepTracker.hpp \
  src/DepView.cpp src/DepView.hpp \
  src/DriverFactory.hpp \
  src/Static/Transforms/EliminateAnnotationsPass.cpp src/Static/Transforms/EliminateAnnotationsPass.hpp \
  src/Static/Transforms/EliminateCASPHIsPass.cpp src/Static/Transforms/EliminateCASPHIsPass.hpp \
  src/Static/Transforms/EliminateCastsPass.cpp src/Static/Transforms/EliminateCastsPass.hpp \
  src/Static/Transforms/EliminateRedundantInstPass.cpp src/Static/Transforms/EliminateRedundantInstPass.hpp \
  src/Static/Transforms/EliminateUnusedCodePass.cpp src/Static/Transforms/EliminateUnusedCodePass.hpp \
  src/Static/Transforms/PromotePause.cpp src/Static/Transforms/PromotePause.hpp \
  src/Static/Transforms/splitRW.cpp src/Static/Transforms/splitRW.hpp \
  src/Static/Transforms/CollectIntrinsics.cpp src/Static/Transforms/CollectIntrinsics.hpp \
  src/Error.hpp \
  src/Static/Transforms/EscapeCheckerPass.cpp src/Static/Transforms/EscapeCheckerPass.hpp \
  src/Event.cpp src/Event.hpp \
  src/EventAttr.hpp \
  src/EventLabel.cpp src/EventLabel.hpp \
  src/Execution.cpp \
  src/ExecutionGraph.cpp src/ExecutionGraph.hpp \
  src/ExternalFunctions.cpp \
  src/Static/Transforms/FunctionInlinerPass.cpp src/Static/Transforms/FunctionInlinerPass.hpp \
  src/GenMCDriver.cpp src/GenMCDriver.hpp \
  src/GraphIterators.hpp \
  src/InstAnnotator.cpp src/InstAnnotator.hpp \
  src/Interpreter.cpp Interpreter.h \
  src/InterpreterEnumAPI.cpp src/InterpreterEnumAPI.hpp \
  src/Static/Transforms/IntrinsicLoweringPass.cpp src/Static/Transforms/IntrinsicLoweringPass.hpp \
  src/LabelVisitor.hpp \
  src/Static/Transforms/LoadAnnotationPass.cpp src/Static/Transforms/LoadAnnotationPass.hpp \
  src/Static/Transforms/LocalSimplifyCFGPass.cpp src/Static/Transforms/LocalSimplifyCFGPass.hpp \
  src/Logger.hpp \
  src/Static/Transforms/LoopUnrollPass.cpp src/Static/Transforms/LoopUnrollPass.hpp \
  src/Static/Transforms/LoopJumpThreadingPass.cpp src/Static/Transforms/LoopJumpThreadingPass.hpp \
  src/Matrix2D.hpp Matrix2D.tcc \
  src/MaximalIterator.hpp \
  src/Static/Transforms/MDataCollectionPass.hpp src/Static/Transforms/MDataCollectionPass.cpp \
  src/MemAccess.hpp src/MemAccess.cpp \
  src/Static/Transforms/MMDetectorPass.cpp src/Static/Transforms/MMDetectorPass.hpp \
  src/Static/Transforms/PromoteMemcpy.cpp \
  src/Static/Transforms/InsertUndefs.cpp \
  src/Static/Transforms/PromoteMemMove.cpp \
  src/ModuleID.hpp \
  src/NameInfo.cpp src/NameInfo.hpp \
  src/Parser.cpp src/Parser.hpp \
  src/Static/LLVMModule.cpp \
  src/Static/LLVMUtils.cpp src/LLVMUtils.hpp \
  src/Static/ModuleInfo.cpp \
  src/Static/Transforms/PromoteMemIntrinsicPass.cpp src/Static/Transforms/PromoteMemIntrinsicPass.hpp \
  src/Static/Transforms/PropagateAssumesPass.cpp src/Static/Transforms/PropagateAssumesPass.hpp \
  src/RADriver.cpp src/RADriver.hpp \
  src/Revisit.cpp src/Revisit.hpp \
  src/RoundBoundDecider.cpp src/RoundBoundDecider.hpp \
  src/SAddr.cpp src/SAddr.hpp \
  src/SAddrAllocator.hpp \
  src/SExpr.tcc src/SExpr.hpp \
  src/SExprVisitor.tcc src/SExprVisitor.hpp \
  src/Stamp.cpp src/Stamp.hpp \
  src/SVal.cpp src/SVal.hpp \
  src/Static/Transforms/SpinAssumePass.cpp src/Static/Transforms/SpinAssumePass.hpp \
  src/WBIterator.hpp \
  src/ThreadInfo.hpp \
  src/ThreadPinner.cpp src/ThreadPinner.hpp \
  src/ThreadPool.cpp src/ThreadPool.hpp \
  src/WorkSet.hpp src/WorkSet.cpp \
  src/value_ptr.hpp \
  src/Verbosity.hpp src/Verbosity.cpp \
  src/VectorClock.hpp src/VectorClock.cpp \
  src/VerificationError.hpp src/VerificationError.cpp \
  src/View.hpp src/View.cpp \
  src/VSet.hpp src/VSet.tcc

TESTS=scripts/fast-driver.sh scripts/randomize-driver.sh

AM_CXXFLAGS = -DINCLUDE_DIR=\"$(pkgincludedir)/$(pkg)/include\" -DSRC_INCLUDE_DIR=\"$(abs_top_srcdir)/include\" $(COVERAGE_CXXFLAGS) -fexceptions
bin_PROGRAMS = genmc
genmc_SOURCES = src/main.cpp
genmc_LDADD = libgenmc.a -lpthread $(COVERAGE_LDFLAGS)

include Makefile.am.coverage

@DX_RULES@
MOSTLYCLEANFILES = DX_CLEANFILES

if LINT
lint-local: $(libgenmc_a_SOURCES) $(genmc_SOURCES)
	-$(CPPLINT) -header-filter='^(?!llvm/).+' -config-file='$(top_srcdir)/.clang-tidy' $^

check-local: lint-local
endif
