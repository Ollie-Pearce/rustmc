#![no_main]
#![feature(start)]
#![feature(thread_spawn_unchecked)]
#![no_builtins]

use std::thread;

extern "C" {
    fn racy_fn();
}

#[start]
#[no_mangle]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    main();
    0
}

#[no_mangle]
fn main() -> i32 {

    thread::spawn( || {
        unsafe{
            racy_fn();
        }
        
    });
        unsafe {
            racy_fn();
        }

    0
}
