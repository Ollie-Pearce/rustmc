; ModuleID = 'my_pthread.c'
source_filename = "my_pthread.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%"core::sync::atomic::AtomicUsize" = type { i64 }

@_ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h541136d3707a013fE = global %"core::sync::atomic::AtomicUsize" {i64 0 }
@_ZN3std6thread10CURRENT_ID17h6a62d35e076fe504E = dso_local global i64 1, align 8
@__dso_handle = dso_local global ptr @__dso_handle, align 8

@"_ZN3std4hash6random11RandomState3new4KEYS29_$u7b$$u7b$constant$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$3VAL17hd69d6c74a2a1681dE" = dso_local global { i64, [2 x i64] } { i64 0, [2 x i64] [i64 0, i64 0] }, align 8
@"_ZN3std6thread5local17LocalKey$LT$T$GT$4with17hd5f9bec89a3fdbd3E" = dso_local global { i64, i64 } { i64 0, i64 0 }, align 8


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
