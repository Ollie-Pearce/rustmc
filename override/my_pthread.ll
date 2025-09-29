; ModuleID = 'my_pthread.c'
source_filename = "my_pthread.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%"unicode_width::tables::Align128<[u8; 256]>" = type { [256 x i8] }
%"unicode_width::tables::Align64<[[u8; 64]; 20]>" = type { [20 x [64 x i8]] }
%"unicode_width::tables::Align32<[[u8; 32]; 184]>" = type { [184 x [32 x i8]] }
%"unicode_width::tables::Align128<[[u8; 128]; 7]>" = type { [7 x [128 x i8]] }
%"std::sync::reentrant_lock::ReentrantLock<core::cell::RefCell<std::io::stdio::StderrRaw>>" = type { %"std::sync::reentrant_lock::Tid", %"std::sys::sync::mutex::futex::Mutex", i32, i64 }
%"std::sync::reentrant_lock::Tid" = type { %"core::sync::atomic::AtomicU64" }
%"core::sync::atomic::AtomicU64" = type { i64 }
%"core::sync::atomic::AtomicU32" = type { i32 }
%"core::sync::atomic::AtomicU8" = type { i8 }
%"core::sync::atomic::AtomicBool" = type { i8 }
%"std::sys::sync::mutex::futex::Mutex" = type { %"core::sync::atomic::AtomicU32" }
%"compiler::Block" = type { i64, [1 x i64] }
%"core::cell::UnsafeCell<std::sys::thread_local::native::lazy::State<core::cell::Cell<core::option::Option<std::sync::mpmc::context::Context>>, ()>>" = type { %"compiler::Block" }
%"std::sys::thread_local::native::lazy::Storage<core::cell::Cell<core::option::Option<std::sync::mpmc::context::Context>>, ()>" = type { %"core::cell::UnsafeCell<std::sys::thread_local::native::lazy::State<core::cell::Cell<core::option::Option<std::sync::mpmc::context::Context>>, ()>>" }
%"proc_macro::bridge::client::HandleCounters" = type { %"core::sync::atomic::AtomicU32", %"core::sync::atomic::AtomicU32", %"core::sync::atomic::AtomicU32", %"core::sync::atomic::AtomicU32" }
%"core::sync::atomic::AtomicUsize" = type { i64 }
%"std::sys::thread_local::native::eager::Storage<core::cell::once::OnceCell<std::thread::Thread>>" = type { ptr, i8, [7 x i8] }
%"core::sync::atomic::AtomicPtr<core::ffi::c_void>" = type { ptr }
%"core::cell::Cell<(usize, usize)>" = type { %"core::cell::UnsafeCell<(usize, usize)>" }
%"core::cell::UnsafeCell<(usize, usize)>" = type { { i64, i64 } }
%"sdd::collector::CollectorRoot" = type { %"core::sync::atomic::AtomicPtr<sdd::collector::Collector>", %"core::sync::atomic::AtomicU8", [7 x i8] }
%"core::sync::atomic::AtomicPtr<sdd::collector::Collector>" = type { ptr }
%"core::sync::atomic::AtomicPtr<parking_lot_core::parking_lot::HashTable>" = type { ptr }

@"_ZN3std4sync4mpmc7context7Context4with7CONTEXT29_$u7b$$u7b$constant$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$3VAL17h1e32d3ce09f1da45E" = thread_local global %"std::sys::thread_local::native::lazy::Storage<core::cell::Cell<core::option::Option<std::sync::mpmc::context::Context>>, ()>" zeroinitializer
@_ZN10proc_macro6bridge6client14HandleCounters3get8COUNTERS17h3e162da0fb433beaE = global %"proc_macro::bridge::client::HandleCounters" zeroinitializer
@_ZN3std2io5stdio6stderr8INSTANCE17h22db7be8253b8d66E = dso_local global %"std::sync::reentrant_lock::ReentrantLock<core::cell::RefCell<std::io::stdio::StderrRaw>>" { %"std::sync::reentrant_lock::Tid" { %"core::sync::atomic::AtomicU64" { i64 0 } }, %"std::sys::sync::mutex::futex::Mutex" { %"core::sync::atomic::AtomicU32" { i32 0 } }, i32 0, i64 0 }, align 8

@"_ZN10proc_macro6bridge6client5state12BRIDGE_STATE29_$u7b$$u7b$constant$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$3VAL17h703a0e7681e6c7efE" = thread_local global ptr null

@_ZN3log20MAX_LOG_LEVEL_FILTER17h520adb782b570d30E = global %"core::sync::atomic::AtomicUsize" {i64 0 }
@_ZN13unicode_width6tables10WIDTH_ROOT17ha1f77ae8a759015fE = local_unnamed_addr global %"unicode_width::tables::Align128<[u8; 256]>" zeroinitializer
@_ZN13unicode_width6tables12WIDTH_MIDDLE17h173e4aab6e3ec159E = local_unnamed_addr global %"unicode_width::tables::Align64<[[u8; 64]; 20]>" zeroinitializer
@_ZN13unicode_width6tables12WIDTH_LEAVES17hbcfbaa8f3ab0c019E = local_unnamed_addr global %"unicode_width::tables::Align32<[[u8; 32]; 184]>" zeroinitializer

@_ZN13unicode_width6tables21EMOJI_MODIFIER_LEAF_017h0241c1d8aedeed83E = global [2 x { i8, i8 }] zeroinitializer
@_ZN13unicode_width6tables21EMOJI_MODIFIER_LEAF_117ha34ae02bb357cdfcE = global [1 x { i8, i8 }] zeroinitializer
@_ZN13unicode_width6tables21EMOJI_MODIFIER_LEAF_217h039e29cda2509620E = global [4 x { i8, i8 }] zeroinitializer
@_ZN13unicode_width6tables21EMOJI_MODIFIER_LEAF_317hef43c3a56f4526c6E = global [9 x { i8, i8 }] zeroinitializer
@_ZN13unicode_width6tables21EMOJI_MODIFIER_LEAF_417h5c548665459732f7E = global [4 x { i8, i8 }] zeroinitializer
@_ZN13unicode_width6tables21EMOJI_MODIFIER_LEAF_517h4579929dd213780dE = global [6 x { i8, i8 }] zeroinitializer
@_ZN13unicode_width6tables21EMOJI_MODIFIER_LEAF_617h9a97258a5d514986E = global [12 x { i8, i8 }] zeroinitializer
@_ZN13unicode_width6tables21EMOJI_MODIFIER_LEAF_717hea8d889f65da9820E = global [2 x { i8, i8 }] zeroinitializer
@_ZN13unicode_width6tables25EMOJI_PRESENTATION_LEAVES17h6532f3356a7c241fE = local_unnamed_addr global %"unicode_width::tables::Align128<[[u8; 128]; 7]>" zeroinitializer

@_ZN13unicode_width6tables24TEXT_PRESENTATION_LEAF_017h1040b1765b73b5f5E = global [4 x { i8, i8 }] zeroinitializer
@_ZN13unicode_width6tables24TEXT_PRESENTATION_LEAF_117hd012b3ced69b0f9fE = global [1 x { i8, i8 }] zeroinitializer
@_ZN13unicode_width6tables24TEXT_PRESENTATION_LEAF_217hf179e866cae1f17cE = global [15 x { i8, i8 }] zeroinitializer
@_ZN13unicode_width6tables24TEXT_PRESENTATION_LEAF_317hd3623533b3713634E = global [10 x { i8, i8 }] zeroinitializer
@_ZN13unicode_width6tables24TEXT_PRESENTATION_LEAF_417hc7f5fb0800ce3e59E = global [3 x { i8, i8 }] zeroinitializer
@_ZN13unicode_width6tables24TEXT_PRESENTATION_LEAF_517hdb61757426f3a970E = global [1 x { i8, i8 }] zeroinitializer
@_ZN13unicode_width6tables24TEXT_PRESENTATION_LEAF_617hdc5acda1354d3542E = global [13 x { i8, i8 }] zeroinitializer
@_ZN13unicode_width6tables24TEXT_PRESENTATION_LEAF_717h933734633221c643E = global [22 x { i8, i8 }] zeroinitializer
@_ZN13unicode_width6tables24TEXT_PRESENTATION_LEAF_817h1237c827c71b6044E = global [4 x { i8, i8 }] zeroinitializer
@_ZN13unicode_width6tables24TEXT_PRESENTATION_LEAF_917hd0590f017456ed3bE = global [10 x { i8, i8 }] zeroinitializer

@_ZN10std_detect6detect5cache5CACHE17ha70d35da6fb9c084E = global [2 x %"std::sync::reentrant_lock::Tid"] zeroinitializer


@_ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17hf10d90ee4e8d5258E = global %"core::sync::atomic::AtomicUsize" {i64 0 }
@_ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h541136d3707a013fE = global %"core::sync::atomic::AtomicUsize" {i64 0 }
@_ZN3std6thread10CURRENT_ID17h6a62d35e076fe504E = dso_local global i64 1, align 8
@"_ZN3std6thread10CURRENT_ID29_$u7b$$u7b$constant$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$3VAL17h72832091327b60bfE" = thread_local global i64 0
@"_ZN3std6thread7CURRENT29_$u7b$$u7b$constant$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$3VAL17ha4638e9f4485ca67E" = thread_local global %"std::sys::thread_local::native::eager::Storage<core::cell::once::OnceCell<std::thread::Thread>>" zeroinitializer


@__dso_handle = dso_local global ptr @__dso_handle, align 8

@"_ZN3std4hash6random11RandomState3new4KEYS29_$u7b$$u7b$constant$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$3VAL17hd69d6c74a2a1681dE" = dso_local global { i64, [2 x i64] } { i64 0, [2 x i64] [i64 0, i64 0] }, align 8
@"_ZN3std4hash6random11RandomState3new4KEYS29_$u7b$$u7b$constant$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$3VAL17h3cea3bcc94e317ffE" = dso_local global { i64, [2 x i64] } { i64 0, [2 x i64] [i64 0, i64 0] }, align 8
@"_ZN3std6thread5local17LocalKey$LT$T$GT$4with17hd5f9bec89a3fdbd3E" = dso_local global { i64, i64 } { i64 0, i64 0 }, align 8
@"_ZN3std6thread5local17LocalKey$LT$T$GT$4with17h591b19b758959997E" = dso_local global { i64, i64 } { i64 0, i64 0 }, align 8

@_ZN16parking_lot_core11parking_lot9HASHTABLE17hb82b50228d40236fE = global <{ [8 x i8] }> zeroinitializer, align 8
@_ZN16parking_lot_core11parking_lot9HASHTABLE17h8a41eac2a6ffc7e7E = global <{ [8 x i8] }> zeroinitializer, align 8
@_ZN16parking_lot_core11parking_lot9HASHTABLE17h2106f294beb32a70E = global %"core::sync::atomic::AtomicPtr<parking_lot_core::parking_lot::HashTable>" zeroinitializer

@_ZN4core7unicode12unicode_data11white_space14WHITESPACE_MAP17h78bfbf1a1051c34cE = dso_local global [256 x i8] [
  i8 2, i8 2, i8 2, i8 2, i8 2, i8 2, i8 2, i8 2,
  i8 2, i8 3, i8 3, i8 1, i8 1, i8 1, i8 0, i8 0,
  i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0,
  i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0,
  i8 1, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0,
  i8 2, i8 2, i8 0, i8 0, i8 0, i8 0, i8 0, i8 2,
  i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0,
  i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0,
  i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0,
  i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0,
  i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0,
  i8 0, i8 0, i8 0, i8 2, i8 0, i8 0, i8 0, i8 0,
  i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0,
  i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0,
  i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0,
  i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0,
  i8 1, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0,
  i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0,
  i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0,
  i8 0, i8 0, i8 0, i8 0, i8 1, i8 0, i8 0, i8 0,
  i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0,
  i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0,
  i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0,
  i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0,
  i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0,
  i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0,
  i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0,
  i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0,
  i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0,
  i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0,
  i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0,
  i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0
], align 1

@_ZN3std3sys3pal4unix14stack_overflow3imp13NEED_ALTSTACK17h752424148a591eb2E = global %"core::sync::atomic::AtomicBool" {i8 0 }
@_ZN3std3sys3pal4unix14stack_overflow3imp9PAGE_SIZE17h64a3e9253a979bf7E = global %"core::sync::atomic::AtomicUsize" {i64 0 }
@_ZN3std3sys3pal4unix14stack_overflow3imp13MAIN_ALTSTACK17h0bc63d12ce7e6ff2E = global %"core::sync::atomic::AtomicPtr<core::ffi::c_void>" { ptr null }
@_ZN3std3sys3pal4unix24ON_BROKEN_PIPE_FLAG_USED17h4c843d6a8af6a147E = global %"core::sync::atomic::AtomicBool" {i8 0}
@"_ZN3std3sys3pal4unix14stack_overflow3imp5GUARD29_$u7b$$u7b$constant$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$3VAL17ha3f8f4a5ee183220E" = thread_local global %"core::cell::Cell<(usize, usize)>" zeroinitializer

@_ZN3sdd9collector11GLOBAL_ROOT17hb069fd236dc18dfcE = global %"sdd::collector::CollectorRoot" zeroinitializer


define dso_local { i64, i64 } @_ZN3std3sys3pal4unix4rand19hashmap_random_keys17hacd20405c8f84a06E() unnamed_addr {
entry:
  %tmp0 = insertvalue { i64, i64 } undef, i64 0, 0
  %tmp1 = insertvalue { i64, i64 } %tmp0, i64 0, 1
  ret { i64, i64 } %tmp1
}

define dso_local { i64, i64 } @_ZN3std3sys3pal4unix4rand19hashmap_random_keys17h5a1fb170aebc4dc9E() unnamed_addr {
entry:
  %tmp0 = insertvalue { i64, i64 } undef, i64 0, 0
  %tmp1 = insertvalue { i64, i64 } %tmp0, i64 0, 1
  ret { i64, i64 } %tmp1
}

; Function Attrs: alwaysinline nounwind uwtable
define dso_local i32 @pthread_create(ptr noalias noundef %__newthread, ptr noalias noundef %__attr, ptr noundef %__start_routine, ptr noalias noundef %__arg) #0 {
entry:
  %__newthread.addr = alloca ptr, align 8
  %__attr.addr = alloca ptr, align 8
  %__start_routine.addr = alloca ptr, align 8
  %__arg.addr = alloca ptr, align 8
  store ptr %__newthread, ptr %__newthread.addr, align 8
  store ptr %__attr, ptr %__attr.addr, align 8
  store ptr %__start_routine, ptr %__start_routine.addr, align 8
  store ptr %__arg, ptr %__arg.addr, align 8
  %0 = load ptr, ptr %__attr.addr, align 8
  %1 = load ptr, ptr %__start_routine.addr, align 8
  %2 = load ptr, ptr %__arg.addr, align 8
  %call = call i32 @__VERIFIER_thread_create(ptr noundef %0, ptr noundef %1, ptr noundef %2) #5
  %conv = sext i32 %call to i64
  %3 = load ptr, ptr %__newthread.addr, align 8
  store i64 %conv, ptr %3, align 8
  ret i32 0
}

; Function Attrs: nounwind
declare i32 @__VERIFIER_thread_create(ptr noundef, ptr noundef, ptr noundef) #1

; Function Attrs: alwaysinline nounwind uwtable
define dso_local i32 @pthread_join(i64 noundef %__th, ptr noundef %__thread_return) #0 {
entry:
  %__th.addr = alloca i64, align 8
  %__thread_return.addr = alloca ptr, align 8
  %__retval = alloca ptr, align 8
  store i64 %__th, ptr %__th.addr, align 8
  store ptr %__thread_return, ptr %__thread_return.addr, align 8
  %0 = load i64, ptr %__th.addr, align 8
  %call = call ptr @__VERIFIER_thread_join(i64 noundef %0) #5
  store ptr %call, ptr %__retval, align 8
  %1 = load ptr, ptr %__thread_return.addr, align 8
  %cmp = icmp ne ptr %1, null
  br i1 %cmp, label %if.then, label %if.end

if.then:                                          ; preds = %entry
  %2 = load ptr, ptr %__retval, align 8
  %3 = load ptr, ptr %__thread_return.addr, align 8
  store ptr %2, ptr %3, align 8
  br label %if.end

if.end:                                           ; preds = %if.then, %entry
  ret i32 0
}

; Function Attrs: nounwind
declare ptr @__VERIFIER_thread_join(i64 noundef) #1

; Function Attrs: alwaysinline nounwind uwtable
define dso_local i32 @pthread_detach(i64 noundef %__th) #0 {
entry:
  %__th.addr = alloca i64, align 8
  store i64 %__th, ptr %__th.addr, align 8
  ret i32 0
}

; Function Attrs: alwaysinline nounwind uwtable
define dso_local i32 @pthread_attr_destroy(ptr noundef %__attr) #0 {
entry:
  %__attr.addr = alloca ptr, align 8
  store ptr %__attr, ptr %__attr.addr, align 8
  ret i32 0
}

; Function Attrs: alwaysinline nounwind uwtable
define dso_local i32 @pthread_attr_init(ptr noundef %__attr) #0 {
entry:
  %__attr.addr = alloca ptr, align 8
  store ptr %__attr, ptr %__attr.addr, align 8
  ret i32 0
}

; Function Attrs: alwaysinline nounwind uwtable
define dso_local i32 @pthread_attr_setstacksize(ptr noundef %__attr, i64 noundef %__stacksize) #0 {
entry:
  %__attr.addr = alloca ptr, align 8
  %__stacksize.addr = alloca i64, align 8
  store ptr %__attr, ptr %__attr.addr, align 8
  store i64 %__stacksize, ptr %__stacksize.addr, align 8
  ret i32 0
}

; Function Attrs: alwaysinline nobuiltin nounwind uwtable
declare void @free(ptr noundef) #2

; Function Attrs: nounwind
declare void @__VERIFIER_free(ptr noundef) #1

; Function Attrs: alwaysinline nobuiltin nounwind allocsize(0) uwtable
declare ptr @malloc(i64 noundef) #3

; Function Attrs: nounwind
declare ptr @__VERIFIER_malloc(i64 noundef) #1

; Function Attrs: alwaysinline nobuiltin nounwind allocsize(1) uwtable
declare ptr @aligned_alloc(i64 noundef, i64 noundef) #4

; Function Attrs: nounwind
declare ptr @__VERIFIER_malloc_aligned(i64 noundef, i64 noundef) #1

; Function Attrs: alwaysinline nounwind uwtable
define dso_local i32 @__cxa_thread_atexit_impl(ptr noundef %func, ptr noundef %ptr, ptr noundef %ptr2) #0 {
entry:
  %func.addr = alloca ptr, align 8
  %ptr.addr = alloca ptr, align 8
  %ptr2.addr = alloca ptr, align 8
  store ptr %func, ptr %func.addr, align 8
  store ptr %ptr, ptr %ptr.addr, align 8
  store ptr %ptr2, ptr %ptr2.addr, align 8
  %0 = load ptr, ptr %func.addr, align 8
  %call = call i32 @__VERIFIER_atexit(ptr noundef %0) #5
  ret i32 %call
}

; Function Attrs: nounwind
declare i32 @__VERIFIER_atexit(ptr noundef) #1



attributes #0 = { alwaysinline nounwind uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { nounwind "frame-pointer"="all" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #2 = { alwaysinline nobuiltin nounwind uwtable "frame-pointer"="all" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #3 = { alwaysinline nobuiltin nounwind allocsize(0) uwtable "frame-pointer"="all" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #4 = { alwaysinline nobuiltin nounwind allocsize(1) uwtable "frame-pointer"="all" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #5 = { nounwind }

!llvm.module.flags = !{!0, !1, !2, !3, !4}
!llvm.ident = !{!5}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 8, !"PIC Level", i32 2}
!2 = !{i32 7, !"PIE Level", i32 2}
!3 = !{i32 7, !"uwtable", i32 2}
!4 = !{i32 7, !"frame-pointer", i32 2}
!5 = !{!"clang version 18.1.7 (https://github.com/llvm/llvm-project.git 8c0fe0d65ed85966c0ac075e896620c55ca95227)"}
