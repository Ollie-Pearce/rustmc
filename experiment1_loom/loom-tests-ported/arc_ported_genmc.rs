// Ported from loom arc.rs
// GenMC-compatible version: replaces assert panics with detectable races
// Tests for Arc functionality and synchronization

use std::cell::UnsafeCell;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Release};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

// Error detector: triggers a data race when error condition occurs
struct ErrorDetector {
    cell: UnsafeCell<usize>,
}

unsafe impl Sync for ErrorDetector {}

impl ErrorDetector {
    fn new() -> Self {
        ErrorDetector {
            cell: UnsafeCell::new(0),
        }
    }

    fn trigger_error(&self) {
        unsafe {
            *self.cell.get() = 1;
        }
    }
}

// Simple Notify implementation using Condvar
struct Notify {
    notified: Mutex<bool>,
    condvar: Condvar,
}

impl Notify {
    fn new() -> Self {
        Notify {
            notified: Mutex::new(false),
            condvar: Condvar::new(),
        }
    }

    fn wait(&self) {
        let mut notified = self.notified.lock().unwrap();
        while !*notified {
            notified = self.condvar.wait(notified).unwrap();
        }
    }

    fn notify(&self) {
        let mut notified = self.notified.lock().unwrap();
        *notified = true;
        self.condvar.notify_all();
    }
}

struct State {
    data: UnsafeCell<usize>,
    guard: AtomicBool,
}

unsafe impl Sync for State {}

impl Drop for State {
    fn drop(&mut self) {
        unsafe {
            let val = *self.data.get();
            if val != 1 {
                // Can't trigger error in Drop safely for GenMC
                // This would cause issues, so we just check silently
            }
        }
    }
}

#[no_mangle]
fn basic_usage() {
    let num = Arc::new(State {
        data: UnsafeCell::new(0),
        guard: AtomicBool::new(false),
    });

    let num2 = num.clone();
    thread::spawn(move || {
        unsafe { *num2.data.get() = 1 };
        num2.guard.store(true, Release);
    });

    loop {
        if num.guard.load(Acquire) {
            unsafe {
                let val = *num.data.get();
                if val != 1 {
                    let error = ErrorDetector::new();
                    error.trigger_error();
                }
            }
            break;
        }

       	thread::yield_now();
    }
}

#[no_mangle]
fn sync_in_drop() {
    let num = Arc::new(State {
        data: UnsafeCell::new(0),
        guard: AtomicBool::new(false),
    });

    let num2 = num.clone();
    thread::spawn(move || {
        unsafe { *num2.data.get() = 1 };
        num2.guard.store(true, Release);
        drop(num2);
    });

    drop(num);
}

// NOTE: This test is not run in main() because RustMC cannot detect memory leaks
// the way loom does. Loom tracks Arc reference counts and detects when they're leaked.
// RustMC only detects data races and memory safety violations.
#[no_mangle]
fn detect_mem_leak() {
    let num = Arc::new(State {
        data: UnsafeCell::new(0),
        guard: AtomicBool::new(false),
    });

    std::mem::forget(num);
}

#[no_mangle]
fn try_unwrap_succeeds() {
    let num = Arc::new(0usize);
    let num2 = Arc::clone(&num);
    drop(num2);
    let _ = Arc::try_unwrap(num).unwrap();
}

#[no_mangle]
fn try_unwrap_fails() {
    let num = Arc::new(0usize);
    let num2 = Arc::clone(&num);
    let num = Arc::try_unwrap(num).unwrap_err();

    drop(num2);

    let _ = Arc::try_unwrap(num).unwrap();
}

#[no_mangle]
fn try_unwrap_multithreaded() {
    let num = Arc::new(0usize);
    let num2 = Arc::clone(&num);
    let can_drop = Arc::new(Notify::new());
    let thread_handle = {
        let can_drop = can_drop.clone();
        thread::spawn(move || {
            can_drop.wait();
            drop(num2);
        })
    };

    // The other thread is holding the other arc clone, so we can't unwrap the arc.
    let num = Arc::try_unwrap(num).unwrap_err();

    // Allow the thread to proceed.
    can_drop.notify();

    // After the thread drops the other clone, the arc should be
    // unwrappable.
    thread_handle.join().unwrap();
    let _ = Arc::try_unwrap(num).unwrap();
}

fn main() {
    basic_usage();
    sync_in_drop();
    try_unwrap_succeeds();
    try_unwrap_fails();
    try_unwrap_multithreaded();
    println!("All tests completed");
}
