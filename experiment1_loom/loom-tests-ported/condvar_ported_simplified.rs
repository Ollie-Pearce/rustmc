// Ported from loom condvar.rs
// Tests condition variable notify_one and notify_all
// EXPECTED: Both should pass (not marked #[should_panic])

use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

struct Inc {
    num: AtomicUsize,
    mutex: Mutex<()>,
    condvar: Condvar,
}

impl Inc {
    fn new() -> Inc {
        Inc {
            num: AtomicUsize::new(0),
            mutex: Mutex::new(()),
            condvar: Condvar::new(),
        }
    }

    fn wait(&self) {
        let mut guard = self.mutex.lock().unwrap();

        loop {
            let val = self.num.load(SeqCst);
            if 1 == val {
                break;
            }

            guard = self.condvar.wait(guard).unwrap();
        }
    }

    fn inc(&self) {
        self.num.store(1, SeqCst);
        drop(self.mutex.lock().unwrap());
        self.condvar.notify_one();
    }

    fn inc_all(&self) {
        self.num.store(1, SeqCst);
        drop(self.mutex.lock().unwrap());
        self.condvar.notify_all();
    }
}

#[no_mangle]
fn notify_one() {
    let inc = Arc::new(Inc::new());
    let inc_clone = inc.clone();
    
    thread::spawn(move || inc_clone.inc());
    inc.wait();
}

#[no_mangle]
fn notify_all() {
    let inc = Arc::new(Inc::new());

    // Create exactly 2 waiters without using Vec
    let inc1 = inc.clone();
    let inc2 = inc.clone();
    let inc3 = inc.clone();
    
    let waiter1 = thread::spawn(move || inc1.wait());
    let waiter2 = thread::spawn(move || inc2.wait());

    thread::spawn(move || inc3.inc_all()).join().expect("inc");

    waiter1.join().expect("waiter1");
    waiter2.join().expect("waiter2");
}

fn main() {
    notify_one();
    notify_all();
    println!("All tests completed");
}