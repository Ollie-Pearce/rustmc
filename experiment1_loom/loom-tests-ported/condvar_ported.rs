// Ported from loom condvar.rs
// Tests condition variable notify_one and notify_all

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

    for _ in 0..1 {
        let inc = inc.clone();
        thread::spawn(move || inc.inc());
    }

    inc.wait();
}

#[no_mangle]
fn notify_all() {
    let inc = Arc::new(Inc::new());

    let mut waiters = Vec::new();
    for _ in 0..2 {
        let inc = inc.clone();
        waiters.push(thread::spawn(move || inc.wait()));
    }

    thread::spawn(move || inc.inc_all()).join().expect("inc");

    for th in waiters {
        th.join().expect("waiter");
    }
}

fn main() {
    notify_one();
    notify_all();
    println!("All tests completed");
}
