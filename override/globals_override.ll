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

@__dso_handle = dso_local global ptr @__dso_handle, align 8
@copy_file_range = global i8 0
@statx =  global i8 0
define i64 @getrandom(ptr %buf, i64 %buflen, i32 %flags) {
entry:
  %cmp = icmp eq i64 %buflen, 0
  br i1 %cmp, label %done, label %loop

loop:
  %i = phi i64 [ 0, %entry ], [ %next, %loop ]
  %ptr = getelementptr i8, ptr %buf, i64 %i
  store i8 0, ptr %ptr, align 1
  %next = add i64 %i, 1
  %cond = icmp ult i64 %next, %buflen
  br i1 %cond, label %loop, label %done

done:
  ret i64 %buflen
}
@environ = local_unnamed_addr global ptr null
@pidfd_spawnp = global i8 0
@pidfd_getpid = global i8 0
@posix_spawn_file_actions_addchdir_np = global i8 0

@__rust_alloc_error_handler_should_panic = local_unnamed_addr global i8 0



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


; Function Attrs: noinline nounwind optnone uwtable
define dso_local ptr @__rust_alloc_zeroed(i64 noundef %0, i64 noundef %1) #5 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca ptr, align 8
  %6 = alloca ptr, align 8
  %7 = alloca i64, align 8
  store i64 %0, ptr %3, align 8
  store i64 %1, ptr %4, align 8
  %8 = load i64, ptr %3, align 8
  %9 = load i64, ptr %4, align 8
  %10 = call ptr @__rust_alloc(i64 noundef %8, i64 noundef %9)
  store ptr %10, ptr %5, align 8
  %11 = load ptr, ptr %5, align 8
  %12 = icmp ne ptr %11, null
  br i1 %12, label %13, label %27

13:                                               ; preds = %2
  %14 = load ptr, ptr %5, align 8
  store ptr %14, ptr %6, align 8
  store i64 0, ptr %7, align 8
  br label %15

15:                                               ; preds = %23, %13
  %16 = load i64, ptr %7, align 8
  %17 = load i64, ptr %3, align 8
  %18 = icmp ult i64 %16, %17
  br i1 %18, label %19, label %26

19:                                               ; preds = %15
  %20 = load ptr, ptr %6, align 8
  %21 = load i64, ptr %7, align 8
  %22 = getelementptr inbounds i8, ptr %20, i64 %21
  store i8 0, ptr %22, align 1
  br label %23

23:                                               ; preds = %19
  %24 = load i64, ptr %7, align 8
  %25 = add i64 %24, 1
  store i64 %25, ptr %7, align 8
  br label %15, !llvm.loop !6

26:                                               ; preds = %15
  br label %27

27:                                               ; preds = %26, %2
  %28 = load ptr, ptr %5, align 8
  ret ptr %28
}

; Function Attrs: noinline nounwind optnone uwtable
declare ptr @__rust_alloc(i64 noundef, i64 noundef) #5

; clock_gettime - stub that zeros out the timespec struct and returns success
; clock_gettime(i32 %clk_id, ptr %tp) -> i32
; struct timespec { time_t tv_sec (8 bytes); long tv_nsec (8 bytes); }
define i32 @clock_gettime(i32 %clk_id, ptr %tp) {
  store i64 0, ptr %tp, align 8
  %nsec_ptr = getelementptr inbounds i8, ptr %tp, i64 8
  store i64 0, ptr %nsec_ptr, align 8
  ret i32 0
}

; memcmp - byte-by-byte memory comparison (based on KLEE's Freestanding implementation)
define i32 @memcmp(ptr %s1, ptr %s2, i64 %n) {
entry:
  %cmp.n = icmp eq i64 %n, 0
  br i1 %cmp.n, label %equal, label %loop

loop:
  %i = phi i64 [ 0, %entry ], [ %i.next, %next ]
  %p1 = getelementptr i8, ptr %s1, i64 %i
  %p2 = getelementptr i8, ptr %s2, i64 %i
  %v1 = load i8, ptr %p1, align 1
  %v2 = load i8, ptr %p2, align 1
  %cmp = icmp eq i8 %v1, %v2
  br i1 %cmp, label %next, label %differ

next:
  %i.next = add i64 %i, 1
  %done = icmp eq i64 %i.next, %n
  br i1 %done, label %equal, label %loop

differ:
  %z1 = zext i8 %v1 to i32
  %z2 = zext i8 %v2 to i32
  %diff = sub i32 %z1, %z2
  ret i32 %diff

equal:
  ret i32 0
}

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
!6 = distinct !{!6, !7}
!7 = !{!"llvm.loop.mustprogress"}
