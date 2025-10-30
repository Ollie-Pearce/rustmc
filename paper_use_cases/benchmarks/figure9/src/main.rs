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
    let data = vec![1, 2, 3, 4];
    let idx = Arc::new(AtomicUsize::new(0));
    let other_idx = idx.clone();

    thread::spawn(move || { 
        other_idx.fetch_add(10, Ordering::SeqCst); 
    });
    
    if idx.load(Ordering::SeqCst) < data.len() {
        unsafe {
            let i = idx.load(Ordering::SeqCst);
            let x = *data.get_unchecked(i); 
        }
    }
    0
}
