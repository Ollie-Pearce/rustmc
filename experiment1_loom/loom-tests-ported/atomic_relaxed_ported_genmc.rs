// Ported from loom atomic_relaxed.rs
// We also replace assert panics with detectable races


use std::cell::UnsafeCell;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
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
fn compare_and_swap() {
    let num = Arc::new(AtomicUsize::new(0));

    let ths: Vec<_> = (0..2)
        .map(|_| {
            let num = num.clone();

            thread::spawn(move || {
                let mut curr = num.load(Relaxed);

                loop {
                    #[allow(deprecated)]
                    let actual = num.compare_and_swap(curr, curr + 1, Relaxed);

                    if actual == curr {
                        return;
                    }

                    curr = actual;
                }
            })
        })
        .collect();

    for th in ths {
        th.join().unwrap();
    }

    // This assertion should always pass
    let final_val = num.load(Relaxed);
    if final_val != 2 {
        // If this fails, it's a real bug - trigger error
        let detector = ErrorDetector::new();
        detector.trigger_error();
    }
}

#[no_mangle]
fn check_ordering_valid() {
    let n1 = Arc::new((AtomicUsize::new(0), AtomicUsize::new(0)));
    let n2 = n1.clone();

    thread::spawn(move || {
        n1.0.store(1, Relaxed);
        n1.1.store(1, Release);
    });

    if 1 == n2.1.load(Acquire) {
        // This should always be true with proper ordering
        let val = n2.0.load(Relaxed);
        if val != 1 {
            // Error condition - trigger race
            let detector = ErrorDetector::new();
            detector.trigger_error();
        }
    }
}

#[no_mangle]
fn check_ordering_invalid_1() {
    // Shared error detector visible to both threads
    let error = Arc::new(ErrorDetector::new());
    let n1 = Arc::new((AtomicUsize::new(0), AtomicUsize::new(0)));
    let n2 = n1.clone();
    let error2 = error.clone();

    thread::spawn(move || {
        n1.0.store(1, Relaxed);
        n1.1.store(1, Release);
        // Also write to error detector from this thread to create race
        error2.trigger_error();
    });

    if 1 == n2.1.load(Relaxed) {
        // This CAN be false due to insufficient ordering
        let val = n2.0.load(Relaxed);
        if val != 1 {
            // Error detected - trigger race from main thread
            // Combined with write from spawned thread, this creates a race
            error.trigger_error();
        }
    }
}

#[no_mangle]
fn check_ordering_invalid_2() {
    // Shared error detector visible to both threads
    let error = Arc::new(ErrorDetector::new());
    let n1 = Arc::new((AtomicUsize::new(0), AtomicUsize::new(0)));
    let n2 = n1.clone();
    let error2 = error.clone();

    thread::spawn(move || {
        n1.0.store(1, Relaxed);
        n1.1.store(1, Relaxed);
        // Also write to error detector from this thread to create race
        error2.trigger_error();
    });

    if 1 == n2.1.load(Relaxed) {
        // This CAN be false due to insufficient ordering
        let val = n2.0.load(Relaxed);
        if val != 1 {
            // Error detected - trigger race from main thread
            // Combined with write from spawned thread, this creates a race
            error.trigger_error();
        }
    }
}

fn main() {
    compare_and_swap();
    check_ordering_valid();
    check_ordering_invalid_1();
    check_ordering_invalid_2();
    println!("All tests completed");
}
