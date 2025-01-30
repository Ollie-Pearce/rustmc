; ModuleID = 'llvm-link'
source_filename = "llvm-link"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@.str = private unnamed_addr constant [15 x i8] c"Hello from C!\0A\00", align 1
@_ZN3std6thread10CURRENT_ID17h6a62d35e076fe504E = dso_local global i64 1, align 8
@__dso_handle = dso_local global ptr @__dso_handle, align 8

; Function Attrs: alwaysinline nounwind nonlazybind
define dso_local i64 @start(i64 %_argc, ptr %_argv) unnamed_addr #0 !dbg !12 {
start:
  call void @llvm.dbg.declare(metadata ptr poison, metadata !22, metadata !DIExpression()), !dbg !25
  call void @llvm.dbg.declare(metadata ptr poison, metadata !23, metadata !DIExpression()), !dbg !26
  %_3 = call i32 @main() #6, !dbg !27
  ret i64 0, !dbg !28
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare void @llvm.dbg.declare(metadata, metadata, metadata) #1

; Function Attrs: alwaysinline nounwind nonlazybind
define dso_local i32 @main() unnamed_addr #0 !dbg !29 {
start:
  call void @hello_from_c() #6, !dbg !33
  ret i32 0, !dbg !34
}

; Function Attrs: noinline nounwind optnone uwtable
define internal void @hello_from_c() #2 {
entry:
  %call = call i32 (ptr, ...) @printf(ptr noundef @.str)
  ret void
}

declare i32 @printf(ptr noundef, ...) #3

; Function Attrs: alwaysinline nounwind uwtable
define dso_local i32 @pthread_create(ptr noalias noundef %__newthread, ptr noalias noundef %__attr, ptr noundef %__start_routine, ptr noalias noundef %__arg) #4 {
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
  %call = call i32 @__VERIFIER_thread_create(ptr noundef %0, ptr noundef %1, ptr noundef %2) #6
  %conv = sext i32 %call to i64
  %3 = load ptr, ptr %__newthread.addr, align 8
  store i64 %conv, ptr %3, align 8
  ret i32 0
}

; Function Attrs: nounwind
declare i32 @__VERIFIER_thread_create(ptr noundef, ptr noundef, ptr noundef) #5

; Function Attrs: alwaysinline nounwind uwtable
define dso_local i32 @pthread_join(i64 noundef %__th, ptr noundef %__thread_return) #4 {
entry:
  %__th.addr = alloca i64, align 8
  %__thread_return.addr = alloca ptr, align 8
  %__retval = alloca ptr, align 8
  store i64 %__th, ptr %__th.addr, align 8
  store ptr %__thread_return, ptr %__thread_return.addr, align 8
  %0 = load i64, ptr %__th.addr, align 8
  %call = call ptr @__VERIFIER_thread_join(i64 noundef %0) #6
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
declare ptr @__VERIFIER_thread_join(i64 noundef) #5

; Function Attrs: alwaysinline nounwind uwtable
define dso_local i32 @pthread_detach(i64 noundef %__th) #4 {
entry:
  %__th.addr = alloca i64, align 8
  store i64 %__th, ptr %__th.addr, align 8
  ret i32 0
}

; Function Attrs: alwaysinline nounwind uwtable
define dso_local i32 @pthread_attr_destroy(ptr noundef %__attr) #4 {
entry:
  %__attr.addr = alloca ptr, align 8
  store ptr %__attr, ptr %__attr.addr, align 8
  ret i32 0
}

; Function Attrs: alwaysinline nounwind uwtable
define dso_local i32 @pthread_attr_init(ptr noundef %__attr) #4 {
entry:
  %__attr.addr = alloca ptr, align 8
  store ptr %__attr, ptr %__attr.addr, align 8
  ret i32 0
}

; Function Attrs: alwaysinline nounwind uwtable
define dso_local i32 @pthread_attr_setstacksize(ptr noundef %__attr, i64 noundef %__stacksize) #4 {
entry:
  %__attr.addr = alloca ptr, align 8
  %__stacksize.addr = alloca i64, align 8
  store ptr %__attr, ptr %__attr.addr, align 8
  store i64 %__stacksize, ptr %__stacksize.addr, align 8
  ret i32 0
}

; Function Attrs: alwaysinline nounwind uwtable
define dso_local i32 @__cxa_thread_atexit_impl(ptr noundef %func, ptr noundef %ptr, ptr noundef %ptr2) #4 {
entry:
  %func.addr = alloca ptr, align 8
  %ptr.addr = alloca ptr, align 8
  %ptr2.addr = alloca ptr, align 8
  store ptr %func, ptr %func.addr, align 8
  store ptr %ptr, ptr %ptr.addr, align 8
  store ptr %ptr2, ptr %ptr2.addr, align 8
  %0 = load ptr, ptr %func.addr, align 8
  %call = call i32 @__VERIFIER_atexit(ptr noundef %0) #6
  ret i32 %call
}

; Function Attrs: nounwind
declare i32 @__VERIFIER_atexit(ptr noundef) #5

attributes #0 = { alwaysinline nounwind nonlazybind "no-builtins" "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #2 = { noinline nounwind optnone uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #3 = { "frame-pointer"="all" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #4 = { alwaysinline nounwind uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #5 = { nounwind "frame-pointer"="all" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #6 = { nounwind }

!llvm.ident = !{!0, !1, !1}
!llvm.dbg.cu = !{!2}
!llvm.module.flags = !{!4, !5, !6, !7, !8, !9, !10, !11}

!0 = !{!"rustc version 1.81.0-dev"}
!1 = !{!"clang version 18.1.7 (https://github.com/llvm/llvm-project.git 8c0fe0d65ed85966c0ac075e896620c55ca95227)"}
!2 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !3, producer: "clang LLVM (rustc version 1.81.0-dev)", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false, nameTableKind: None)
!3 = !DIFile(filename: "src/main.rs/@/8qt8ncmvtsl7ypv0obl2foh3j", directory: "/home/zjac281/Desktop/RustMC/genmc_for_rust/rust_benchmark/mixed_codebases/race_free_test")
!4 = !{i32 8, !"PIC Level", i32 2}
!5 = !{i32 7, !"PIE Level", i32 2}
!6 = !{i32 2, !"RtLibUseGOT", i32 1}
!7 = !{i32 2, !"Dwarf Version", i32 4}
!8 = !{i32 2, !"Debug Info Version", i32 3}
!9 = !{i32 1, !"wchar_size", i32 4}
!10 = !{i32 7, !"uwtable", i32 2}
!11 = !{i32 7, !"frame-pointer", i32 2}
!12 = distinct !DISubprogram(name: "start", scope: !14, file: !13, line: 13, type: !15, scopeLine: 13, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !2, templateParams: !24, retainedNodes: !21)
!13 = !DIFile(filename: "src/main.rs", directory: "/home/zjac281/Desktop/RustMC/genmc_for_rust/rust_benchmark/mixed_codebases/race_free_test", checksumkind: CSK_MD5, checksum: "7d4e0b0eb6e640ad0ada7fb5a6530fcd")
!14 = !DINamespace(name: "race_free_test", scope: null)
!15 = !DISubroutineType(types: !16)
!16 = !{!17, !17, !18}
!17 = !DIBasicType(name: "isize", size: 64, encoding: DW_ATE_signed)
!18 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const *const u8", baseType: !19, size: 64, align: 64, dwarfAddressSpace: 0)
!19 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const u8", baseType: !20, size: 64, align: 64, dwarfAddressSpace: 0)
!20 = !DIBasicType(name: "u8", size: 8, encoding: DW_ATE_unsigned)
!21 = !{!22, !23}
!22 = !DILocalVariable(name: "_argc", arg: 1, scope: !12, file: !13, line: 13, type: !17)
!23 = !DILocalVariable(name: "_argv", arg: 2, scope: !12, file: !13, line: 13, type: !18)
!24 = !{}
!25 = !DILocation(line: 13, column: 10, scope: !12)
!26 = !DILocation(line: 13, column: 24, scope: !12)
!27 = !DILocation(line: 14, column: 5, scope: !12)
!28 = !DILocation(line: 16, column: 2, scope: !12)
!29 = distinct !DISubprogram(name: "main", scope: !14, file: !13, line: 20, type: !30, scopeLine: 20, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !2, templateParams: !24)
!30 = !DISubroutineType(types: !31)
!31 = !{!32}
!32 = !DIBasicType(name: "i32", size: 32, encoding: DW_ATE_signed)
!33 = !DILocation(line: 23, column: 9, scope: !29)
!34 = !DILocation(line: 26, column: 2, scope: !29)
