; ModuleID = 'llvm-link'
source_filename = "llvm-link"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@_ZN3std6thread10CURRENT_ID17h6a62d35e076fe504E = dso_local global i64 1, align 8
@__dso_handle = dso_local global ptr @__dso_handle, align 8
@_ZN10std_detect6detect5cache5CACHE17ha70d35da6fb9c084E = local_unnamed_addr global <{ [16 x i8] }> zeroinitializer, align 8
@_ZN3std2io5stdio6stderr8INSTANCE17h22db7be8253b8d66E = global <{ [24 x i8] }> zeroinitializer, align 8
@_ZN3std3sys3pal4unix4rand3imp20getrandom_fill_bytes21GETRANDOM_UNAVAILABLE17h04e842afe27544adE = local_unnamed_addr global <{ [1 x i8] }> zeroinitializer, align 1
@_ZN3std3sys3pal4unix4rand3imp9getrandom23GRND_INSECURE_AVAILABLE17hbc3f149190aeaa63E = local_unnamed_addr global <{ [1 x i8] }> <{ [1 x i8] c"\01" }>, align 1
@"_ZN3std4hash6random11RandomState3new4KEYS29_$u7b$$u7b$constant$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$3VAL17h3cea3bcc94e317ffE" = thread_local local_unnamed_addr global <{ [8 x i8], [16 x i8] }> <{ [8 x i8] zeroinitializer, [16 x i8] undef }>, align 8
@"_ZN3std4sync4mpmc7context7Context4with7CONTEXT29_$u7b$$u7b$constant$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$3VAL17h1e32d3ce09f1da45E" = thread_local local_unnamed_addr global <{ [8 x i8], [8 x i8] }> <{ [8 x i8] zeroinitializer, [8 x i8] undef }>, align 8
@"_ZN3std6thread10CURRENT_ID29_$u7b$$u7b$constant$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$3VAL17h72832091327b60bfE" = thread_local global <{ [8 x i8] }> zeroinitializer, align 8
@"_ZN3std6thread7CURRENT29_$u7b$$u7b$constant$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$3VAL17ha4638e9f4485ca67E" = thread_local global <{ [9 x i8], [7 x i8] }> <{ [9 x i8] zeroinitializer, [7 x i8] undef }>, align 8
@_ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h541136d3707a013fE = global <{ [8 x i8] }> zeroinitializer, align 8
@"_ZN10proc_macro6bridge6client5state12BRIDGE_STATE29_$u7b$$u7b$constant$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$3VAL17h703a0e7681e6c7efE" = thread_local global <{ [8 x i8] }> zeroinitializer, align 8
@_ZN10proc_macro6bridge6client14HandleCounters3get8COUNTERS17h3e162da0fb433beaE = local_unnamed_addr global <{ [16 x i8] }> <{ [16 x i8] c"\01\00\00\00\01\00\00\00\01\00\00\00\01\00\00\00" }>, align 4

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
  %call = call i32 @__VERIFIER_thread_create(ptr noundef %0, ptr noundef %1, ptr noundef %2) #2
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
  %call = call ptr @__VERIFIER_thread_join(i64 noundef %0) #2
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
  %call = call i32 @__VERIFIER_atexit(ptr noundef %0) #2
  ret i32 %call
}

; Function Attrs: nounwind
declare i32 @__VERIFIER_atexit(ptr noundef) #1

attributes #0 = { alwaysinline nounwind uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { nounwind "frame-pointer"="all" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #2 = { nounwind }

!llvm.ident = !{!0}
!llvm.module.flags = !{!1, !2, !3, !4, !5}

!0 = !{!"clang version 18.1.7 (https://github.com/llvm/llvm-project.git 8c0fe0d65ed85966c0ac075e896620c55ca95227)"}
!1 = !{i32 1, !"wchar_size", i32 4}
!2 = !{i32 8, !"PIC Level", i32 2}
!3 = !{i32 7, !"PIE Level", i32 2}
!4 = !{i32 7, !"uwtable", i32 2}
!5 = !{i32 7, !"frame-pointer", i32 2}
