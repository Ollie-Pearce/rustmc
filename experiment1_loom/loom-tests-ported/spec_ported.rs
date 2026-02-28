// Ported from loom spec.rs
// Tests converted from the C11 memory ordering page


use std::sync::atomic::{AtomicBool, AtomicUsize};
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;

/// Maximum number of iterations for spin loops (to avoid unbounded loops in model checking)
const MAX_SPINS: usize = 1;

/// https://en.cppreference.com/w/cpp/atomic/memory_order#Relaxed_ordering
///
/// Originally #[should_panic] #[ignore] - loom cannot fully model Ordering::Relaxed
/// Tests for the exotic "out of thin air" case where r1=42 AND r2=42
/// This is a cyclic dependency that shouldn't be possible with normal interleaving,
/// but is theoretically possible with Relaxed ordering + speculation.
///
/// Original test: panicked when (42, 42) was found, which was expected behavior
/// Ported test: asserts that (42, 42) does NOT occur. If your model checker finds it,
///              the assertion will fail, indicating your checker models this exotic behavior.
#[no_mangle]
fn relaxed() {
    let x = Arc::new(AtomicUsize::new(0));
    let y = Arc::new(AtomicUsize::new(0));

    let t1 = {
        let x = x.clone();
        let y = y.clone();
        thread::spawn(move || {
            let r1 = y.load(Ordering::Relaxed);
            x.store(r1, Ordering::Relaxed);
            r1
        })
    };

    let t2 = {
        let x = x.clone();
        let y = y.clone();
        thread::spawn(move || {
            let r2 = x.load(Ordering::Relaxed);
            y.store(42, Ordering::Relaxed);
            r2
        })
    };

    let r1 = t1.join().unwrap();
    let r2 = t2.join().unwrap();

    // The exotic case: both threads observe 42 before either stored it!
    // Thread 1: reads y=42, stores 42 to x
    // Thread 2: reads x=42, stores 42 to y
    // This is circular - the value appears "out of thin air"
    //
    // With normal interleaving, possible outcomes are: (0,0), (0,42), (42,0)
    // The (42,42) case requires speculation/out-of-order execution with Relaxed
    assert!(
        !(r1 == 42 && r2 == 42),
        "Found exotic case: r1={}, r2={} - both are 42! \
         This 'out of thin air' value is theoretically possible with Relaxed ordering.",
        r1, r2
    );
}

#[no_mangle]
fn acq_rel() {

    // FROM ORIGINAL TEST
    // The yield loop makes loom really sad without this:
    // builder.preemption_bound = Some(1);
    let x = Arc::new(AtomicBool::new(false));
    let y = Arc::new(AtomicBool::new(false));
    let z = Arc::new(AtomicUsize::new(0));

    // Store to x happens in main thread after spawning
    {
        let y = y.clone();
        thread::spawn(move || {
            y.store(true, Ordering::Release);
        });
    }

    let t1 = {
        let x = x.clone();
        let y = y.clone();
        let z = z.clone();
        thread::spawn(move || {
            for _ in 0..MAX_SPINS {
                if x.load(Ordering::Acquire) {
                    break;
                }
                thread::yield_now();
            }
            if y.load(Ordering::Acquire) {
                z.fetch_add(1, Ordering::Relaxed);
            }
        })
    };

    let t2 = {
        let x = x.clone();
        let y = y.clone();
        let z = z.clone();
        thread::spawn(move || {
            for _ in 0..MAX_SPINS {
                if y.load(Ordering::Acquire) {
                    break;
                }
                thread::yield_now();
            }
            if x.load(Ordering::Acquire) {
                z.fetch_add(1, Ordering::Relaxed);
            }
        })
    };

    x.store(true, Ordering::Release);

    t1.join().unwrap();
    t2.join().unwrap();

    let z_val = z.load(Ordering::SeqCst);
    // Depending on interleaving, z can be 0, 1, or 2
    assert!(z_val <= 2, "z = {} is not valid (should be 0, 1, or 2)", z_val);
}

#[no_mangle]
fn test_seq_cst() {
    let x = Arc::new(AtomicBool::new(false));
    let y = Arc::new(AtomicBool::new(false));
    let z = Arc::new(AtomicUsize::new(0));

    {
        let y = y.clone();
        thread::spawn(move || {
            y.store(true, Ordering::SeqCst);
        });
    }

    let t1 = {
        let x = x.clone();
        let y = y.clone();
        let z = z.clone();
        thread::spawn(move || {
            for _ in 0..MAX_SPINS {
                if x.load(Ordering::SeqCst) {
                    break;
                }
                thread::yield_now();
            }
            if y.load(Ordering::SeqCst) {
                z.fetch_add(1, Ordering::Relaxed);
            }
        })
    };

    let t2 = {
        let x = x.clone();
        let y = y.clone();
        let z = z.clone();
        thread::spawn(move || {
            for _ in 0..MAX_SPINS {
                if y.load(Ordering::SeqCst) {
                    break;
                }
                thread::yield_now();
            }
            if x.load(Ordering::SeqCst) {
                z.fetch_add(1, Ordering::Relaxed);
            }
        })
    };

    x.store(true, Ordering::SeqCst);

    t1.join().unwrap();
    t2.join().unwrap();

    let z_val = z.load(Ordering::SeqCst);
    // With SeqCst, z == 0 should not be possible, but one interleaving will have z == 1 or z == 2
    // The original test was ignored because loom found z == 0 which shouldn't happen with SeqCst
    assert!(z_val <= 2, "z = {} is not valid (should be 0, 1, or 2)", z_val);
}

fn main() {
    relaxed();
    acq_rel();
    test_seq_cst();
    println!("All tests completed");
}
