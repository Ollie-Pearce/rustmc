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


static atomic_forty_two: AtomicU64 = AtomicU64::new(0);
static mut nax: u64 = 0;

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

    unsafe{
        thread::spawn( || {
            nax = 1; // A
            atomic_forty_two.store(1, Ordering::Release); // B
        
        });
        
        thread::spawn( || {
            if ( atomic_forty_two.load(Ordering::Acquire) == 1){ // C
                atomic_forty_two.store(2, Ordering::Relaxed); // D
            }
        });
        
        if ( atomic_forty_two.load(Ordering::Acquire) == 2){ // E
            assert!(nax==1);
        }
    }
    0
}

