// Ported from loom atomic_int.rs
// Tests for various atomic integer operations


use std::sync::atomic::*;
use std::sync::atomic::Ordering::SeqCst;

const NUM_A: u64 = 11641914933775430211;
const NUM_B: u64 = 13209405719799650717;

// Define unique functions for atomic_usize tests
#[no_mangle]
fn xor() {
    let a: usize = NUM_A as usize;
    let b: usize = NUM_B as usize;

    let atomic = AtomicUsize::new(a);
    let prev = atomic.fetch_xor(b, SeqCst);

    assert_eq!(a, prev, "prev did not match");
    assert_eq!(a ^ b, atomic.load(SeqCst), "load failed");
}

#[no_mangle]
fn max() {
    let a: usize = NUM_A as usize;
    let b: usize = NUM_B as usize;

    let atomic = AtomicUsize::new(a);
    let prev = atomic.fetch_max(b, SeqCst);

    assert_eq!(a, prev, "prev did not match");
    assert_eq!(a.max(b), atomic.load(SeqCst), "load failed");
}

#[no_mangle]
fn min() {
    let a: usize = NUM_A as usize;
    let b: usize = NUM_B as usize;

    let atomic = AtomicUsize::new(a);
    let prev = atomic.fetch_min(b, SeqCst);

    assert_eq!(a, prev, "prev did not match");
    assert_eq!(a.min(b), atomic.load(SeqCst), "load failed");
}

#[no_mangle]
fn compare_exchange() {
    let a: usize = NUM_A as usize;
    let b: usize = NUM_B as usize;

    let atomic = AtomicUsize::new(a);
    assert_eq!(Err(a), atomic.compare_exchange(b, a, SeqCst, SeqCst));
    assert_eq!(Ok(a), atomic.compare_exchange(a, b, SeqCst, SeqCst));

    assert_eq!(b, atomic.load(SeqCst));
}

#[no_mangle]
fn compare_exchange_weak() {
    let a: usize = NUM_A as usize;
    let b: usize = NUM_B as usize;

    let atomic = AtomicUsize::new(a);
    assert_eq!(Err(a), atomic.compare_exchange_weak(b, a, SeqCst, SeqCst));
    assert_eq!(Ok(a), atomic.compare_exchange_weak(a, b, SeqCst, SeqCst));

    assert_eq!(b, atomic.load(SeqCst));
}

#[no_mangle]
fn fetch_update() {
    let a: usize = NUM_A as usize;
    let b: usize = NUM_B as usize;

    let atomic = AtomicUsize::new(a);
    assert_eq!(Ok(a), atomic.fetch_update(SeqCst, SeqCst, |_| Some(b)));
    assert_eq!(Err(b), atomic.fetch_update(SeqCst, SeqCst, |_| None));
    assert_eq!(b, atomic.load(SeqCst));
}

fn main() {
    xor();
    max();
    min();
    compare_exchange();
    fetch_update();

    println!("All tests completed");
}
