

#![no_main]
#![feature(start)]
#![feature(thread_spawn_unchecked)]
#![no_builtins]
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::atomic;
use std::sync::atomic::ATOMIC_BOOL_INIT;
use std::io::Error;
use libc::c_long;
use std::thread;
use std::sync::{Once, ONCE_INIT};
use std::sync::atomic::AtomicI64;
use std::process::abort;

#[start]
#[no_mangle]
#[inline(always)]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    main();
    0
}

fn getrand(_buf: &mut [u64]) -> libc::c_long { 1 }

static CHECKED: AtomicI64 = AtomicI64::new(0);
static AVAILABLE: AtomicI64 = AtomicI64::new(0);
fn is_getrand_available() -> i64 {
    if (CHECKED.load(Ordering::Relaxed) == 0 ){
        let mut buf: [u64; 0] = [];
        let result = getrand(&mut buf);
        let available = if result == -1 { abort() } else { 1 };
        AVAILABLE.store(available, Ordering::Relaxed);
        CHECKED.store(1, Ordering::Relaxed); 
        available
    } else { AVAILABLE.load(Ordering::Relaxed) } 
}

#[no_mangle]
#[inline(always)]
fn main() {
    let t1 = 
    thread::spawn(||{
      is_getrand_available()
    });
    let t2 = 
    thread::spawn(||{
      is_getrand_available()
    });

    let r1 = t1.join().unwrap();
    let r2 = t2.join().unwrap();

    assert_eq!(r1, r2);
}
/* 
[2025-01-29T16:05:43Z WARN  lockbud::callbacks] [
      {
        "AtomicityViolation": {
          "bug_kind": "AtomicityViolation",
          "possibility": "Possibly",
          "diagnosis": {
            "fn_name": "is_getrand_available",
            "atomic_reader": "src/main.rs:21:9: 21:40 (#0)",
            "atomic_writer": "src/main.rs:26:9: 26:44 (#0)",
            "dep_kind": "Control"
          },
          "explanation": "atomic::store is data/control dependent on atomic::load"
        }
      }
    ]*/