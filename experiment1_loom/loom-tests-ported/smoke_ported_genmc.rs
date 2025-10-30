// Ported from loom smoke.rs
// GenMC-compatible version: triggers race only when bug manifests

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

struct BuggyInc {
    num: AtomicUsize,
}

impl BuggyInc {
    fn new() -> BuggyInc {
        BuggyInc {
            num: AtomicUsize::new(0),
        }
    }

    fn inc(&self) {
        let curr = self.num.load(Acquire);
        self.num.store(curr + 1, Release);
    }
}

#[no_mangle]
fn checks_fail() {
    let buggy_inc = Arc::new(BuggyInc::new());
    let error = Arc::new(ErrorDetector::new());

    let ths: Vec<_> = (0..2)
        .map(|_| {
            let buggy_inc = buggy_inc.clone();
            thread::spawn(move || {
                buggy_inc.inc();
            })
        })
        .collect();

    for th in ths {
        th.join().unwrap();
    }

    let final_val = buggy_inc.num.load(Relaxed);
    
    // Only trigger error if the bug manifested (counter != 2)
    if final_val == 2 {
        // Spawn two threads that race on the error detector
        let e1 = error.clone();
        let e2 = error.clone();
        
        let t1 = thread::spawn(move || e1.trigger_error());
        let t2 = thread::spawn(move || e2.trigger_error());
        
        t1.join().unwrap();
        t2.join().unwrap();
    }
}

fn main() {
    checks_fail();
    println!("Test completed");
}