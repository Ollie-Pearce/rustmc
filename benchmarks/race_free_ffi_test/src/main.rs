#![no_main]
#![feature(start)]
#![feature(thread_spawn_unchecked)]
#![no_builtins]

extern "C" {
    fn hello_from_c();
}

#[start]
#[no_mangle]
#[inline(always)]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    main();
    0
}

#[no_mangle]
#[inline(always)]
pub extern "C" fn main() -> i32{
    // Call the C function. It's unsafe because it relies on an external function.
    unsafe {
        hello_from_c();
    }
    0
}

