// Ported from loom mutex.rs

use std::cell::UnsafeCell;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::{Arc, Mutex};
use std::thread;

#[no_mangle]
fn mutex_enforces_mutal_exclusion() {
    let data = Arc::new((Mutex::new(0), AtomicUsize::new(0)));

    let ths: Vec<_> = (0..2)
        .map(|_| {
            let data = data.clone();

            thread::spawn(move || {
                let mut locked = data.0.lock().unwrap();

                let prev = data.1.fetch_add(1, SeqCst);
                assert_eq!(prev, *locked);
                *locked += 1;
            })
        })
        .collect();

    for th in ths {
        th.join().unwrap();
    }

    let locked = data.0.lock().unwrap();

    assert_eq!(*locked, data.1.load(SeqCst));
}

#[no_mangle]
fn mutex_establishes_seq_cst() {
    struct Data {
        cell: UnsafeCell<usize>,
        flag: Mutex<bool>,
    }

    unsafe impl Sync for Data {}

    let data = Arc::new(Data {
        cell: UnsafeCell::new(0),
        flag: Mutex::new(false),
    });

    {
        let data = data.clone();

        thread::spawn(move || {
            unsafe { *data.cell.get() = 1 };
            *data.flag.lock().unwrap() = true;
        });
    }

    let flag = *data.flag.lock().unwrap();

    if flag {
        let v = unsafe { *data.cell.get() };
        assert_eq!(v, 1);
    }
}

#[no_mangle]
fn mutex_into_inner() {
    let lock = Arc::new(Mutex::new(0));

    let ths: Vec<_> = (0..2)
        .map(|_| {
            let lock = lock.clone();

            thread::spawn(move || {
                *lock.lock().unwrap() += 1;
            })
        })
        .collect();

    for th in ths {
        th.join().unwrap();
    }

    let lock = Arc::try_unwrap(lock).unwrap().into_inner().unwrap();
    assert_eq!(lock, 2);
}

fn main() {
    mutex_enforces_mutal_exclusion();
    mutex_establishes_seq_cst();
    mutex_into_inner();
    println!("All tests completed");
}
