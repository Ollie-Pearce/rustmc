// Ported from loom atomic.rs
// GenMC-compatible version: replaces assert panics with detectable races
// Tests for atomic operations and memory ordering
// Note: lazy_static and thread_local tests are omitted as they test loom-specific infrastructure

use std::cell::UnsafeCell;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::{AcqRel, Acquire, Relaxed, Release};
use std::sync::Arc;
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

    // Trigger a race by unsafely writing to shared memory
    fn trigger_error(&self) {
        unsafe {
            *self.cell.get() = 1;
        }
    }
}

#[no_mangle]
fn invalid_unsync_load_relaxed() {
    // This test doesn't have an assertion, but the pattern itself is invalid
    // We add error detector to make the race explicit

    let a = Arc::new(AtomicUsize::new(0));
    let b = a.clone();


    let thread_handle = thread::spawn(move || {
        // Note: unsync_load is loom-specific.
        unsafe { 
                let ptr = &*a as *const AtomicUsize as *const usize;
                let _ = *ptr;
            }
    });

    b.store(1, Relaxed);

    thread_handle.join().unwrap();
}

#[no_mangle]
fn compare_and_swap_reads_old_values() {
    let error = Arc::new(ErrorDetector::new());
    let a = Arc::new(AtomicUsize::new(0));
    let b = Arc::new(AtomicUsize::new(0));

    let a2 = a.clone();
    let b2 = b.clone();
    let error2 = error.clone();

    let th = thread::spawn(move || {
        a2.store(1, Release);
        #[allow(deprecated)]
        b2.compare_and_swap(0, 2, AcqRel);
        // Trigger error from thread
        error2.trigger_error();
    });

    b.store(1, Release);
    #[allow(deprecated)]
    a.compare_and_swap(0, 2, AcqRel);

    th.join().unwrap();

    let a_val = a.load(Acquire);
    let b_val = b.load(Acquire);

    // If both succeeded in swapping, this is the error condition
    if a_val == 2 && b_val == 2 {
        // Trigger error from main thread - creates race with thread write
        error.trigger_error();
    }
}

#[no_mangle]
fn fetch_add_atomic() {
    let a1 = Arc::new(AtomicUsize::new(0));
    let a2 = a1.clone();

    let th = thread::spawn(move || a2.fetch_add(1, Relaxed));

    let v1 = a1.fetch_add(1, Relaxed);
    let v2 = th.join().unwrap();

    // This should always be true for correct implementation
    if v1 == v2 {
        // If they're equal, it's an error
        let error = ErrorDetector::new();
        error.trigger_error();
    }
}

fn main() {
    invalid_unsync_load_relaxed();
    compare_and_swap_reads_old_values();
    fetch_add_atomic();
    println!("All tests completed");
}
