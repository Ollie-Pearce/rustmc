#![no_main]
#![feature(start)]
#![feature(thread_spawn_unchecked)]
#![no_builtins]
use std::ptr;
use std::thread;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::AtomicU8;
use std::arch::asm;
use std::time::Duration;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
use std::ptr::addr_of_mut;

#[start]
#[no_mangle]
#[inline(always)]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    main();
    0
}

#[no_mangle]
#[inline(always)]
fn main() -> i32 {
    let mut x: i64 = 2;

    let x_ptr = &mut x as *mut i64;
    
    unsafe {
        let x_ptr_2: &mut i64 = &mut *x_ptr; 

        let handle = thread::spawn(move || {
            *x_ptr_2 = 5; 
        });

        *x_ptr = 10;

        handle.join().unwrap();
    }
    let y = x; 
    0
}
