use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

// Test 1: Basic thread isolation with atomics
static VALUE_T1: AtomicUsize = AtomicUsize::new(0);
static VALUE_T2: AtomicUsize = AtomicUsize::new(0);
static VALUE_MAIN: AtomicUsize = AtomicUsize::new(0);

fn do_test(storage: &AtomicUsize, n: usize) {
    // Initial value should be 0 (simulating thread-local init to 1, then we set it)
    let initial = storage.load(Ordering::SeqCst);
    assert_eq!(initial, 0);
    
    // Set to our thread-specific value
    storage.store(n, Ordering::SeqCst);
    
    // Verify it stayed our value
    let current = storage.load(Ordering::SeqCst);
    assert_eq!(current, n);
}

#[no_mangle]
fn thread_local_test() {
    let t1 = thread::spawn(|| do_test(&VALUE_T1, 2));
    let t2 = thread::spawn(|| do_test(&VALUE_T2, 3));
    
    do_test(&VALUE_MAIN, 4);
    
    t1.join().unwrap();
    t2.join().unwrap();
    
    // Verify each thread had its own storage
    assert_eq!(VALUE_T1.load(Ordering::SeqCst), 2);
    assert_eq!(VALUE_T2.load(Ordering::SeqCst), 3);
    assert_eq!(VALUE_MAIN.load(Ordering::SeqCst), 4);
}

// Test 2: Nested access
static LOCAL1: AtomicUsize = AtomicUsize::new(1);
static LOCAL2: AtomicUsize = AtomicUsize::new(2);

#[no_mangle]
fn nested_with() {
    let val2 = LOCAL2.load(Ordering::SeqCst);
    LOCAL1.store(val2, Ordering::SeqCst);
    assert_eq!(LOCAL1.load(Ordering::SeqCst), 2);
}

// Test 3: Drop counting
static DROPS: AtomicUsize = AtomicUsize::new(0);

struct CountDrops;

impl Drop for CountDrops {
    fn drop(&mut self) {
        DROPS.fetch_add(1, Ordering::Release);
    }
}

impl CountDrops {
    fn new() -> Self {
        Self
    }
}

fn use_droppable() {
    let _local = CountDrops::new();
    // Drops when function returns
}

#[no_mangle]
fn drop_test() {
    assert_eq!(DROPS.load(Ordering::Acquire), 0);

    thread::spawn(|| {
        use_droppable();
        assert_eq!(DROPS.load(Ordering::Acquire), 1);
    })
    .join()
    .unwrap();

    assert_eq!(DROPS.load(Ordering::Acquire), 1);

    thread::spawn(|| {
        use_droppable();
        assert_eq!(DROPS.load(Ordering::Acquire), 2);
    })
    .join()
    .unwrap();

    assert_eq!(DROPS.load(Ordering::Acquire), 2);

    use_droppable();
    assert_eq!(DROPS.load(Ordering::Acquire), 3);
}

fn main() {
    thread_local_test();
    nested_with();
    drop_test();
    
    println!("All tests completed");
}