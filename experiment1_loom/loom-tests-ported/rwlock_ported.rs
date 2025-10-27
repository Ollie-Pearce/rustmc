// Ported from loom rwlock.rs
// Tests for RwLock operations


use std::sync::{Arc, RwLock, TryLockError};
use std::thread;

#[no_mangle]
fn rwlock_read_one() {
    let lock = Arc::new(RwLock::new(1));
    let c_lock = lock.clone();

    let n = lock.read().unwrap();
    assert_eq!(*n, 1);

    thread::spawn(move || {
        let r = c_lock.read();
        assert!(r.is_ok());
    })
    .join()
    .unwrap();
}

#[no_mangle]
fn rwlock_read_two_write_one() {
    let lock = Arc::new(RwLock::new(1));

    for _ in 0..2 {
        let lock = lock.clone();

        thread::spawn(move || {
            let _l = lock.read().unwrap();

            thread::yield_now();
        });
    }

    let _l = lock.write().unwrap();
    thread::yield_now();
}

#[no_mangle]
fn rwlock_write_three() {
    let lock = Arc::new(RwLock::new(1));

    for _ in 0..2 {
        let lock = lock.clone();
        thread::spawn(move || {
            let _l = lock.write().unwrap();

            thread::yield_now();
        });
    }

    let _l = lock.write().unwrap();
    thread::yield_now();
}

#[no_mangle]
fn rwlock_write_then_try_write() {
    let lock = Arc::new(RwLock::new(1));

    let _l1 = lock.write().unwrap();

    assert!(matches!(
        lock.try_write(),
        Err(TryLockError::WouldBlock)
    ));
}

#[no_mangle]
fn rwlock_write_then_try_read() {
    let lock = Arc::new(RwLock::new(1));

    let _l1 = lock.write().unwrap();

    assert!(matches!(lock.try_read(), Err(TryLockError::WouldBlock)));
}

#[no_mangle]
fn rwlock_read_then_try_write() {
    let lock = Arc::new(RwLock::new(1));

    let _l1 = lock.write().unwrap();

    assert!(matches!(
        lock.try_write(),
        Err(TryLockError::WouldBlock)
    ));
}

#[no_mangle]
fn rwlock_try_read() {
    let lock = RwLock::new(1);

    match lock.try_read() {
        Ok(n) => assert_eq!(*n, 1),
        Err(_) => unreachable!(),
    };
}

#[no_mangle]
fn rwlock_write() {
    let lock = RwLock::new(1);

    let mut n = lock.write().unwrap();
    *n = 2;

    assert!(lock.try_read().is_err());
}

#[no_mangle]
fn rwlock_try_write() {
    let lock = RwLock::new(1);

    let n = lock.read().unwrap();
    assert_eq!(*n, 1);

    assert!(lock.try_write().is_err());
}

#[no_mangle]
fn rwlock_into_inner() {
    let lock = Arc::new(RwLock::new(0));

    let ths: Vec<_> = (0..2)
        .map(|_| {
            let lock = lock.clone();

            thread::spawn(move || {
                *lock.write().unwrap() += 1;
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
    rwlock_read_one();
    rwlock_read_two_write_one();
    rwlock_write_three();
    rwlock_write_then_try_write();
    rwlock_write_then_try_read();
    rwlock_read_then_try_write();
    rwlock_try_read();
    rwlock_write();
    rwlock_try_write();
    rwlock_into_inner();
    println!("All tests completed");
}
