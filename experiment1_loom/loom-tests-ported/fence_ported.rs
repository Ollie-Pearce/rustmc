// Ported from loom fence.rs
// Tests for memory fences

use std::cell::UnsafeCell;
use std::sync::atomic::{fence, AtomicBool};
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release, SeqCst};
use std::sync::Arc;
use std::thread;

// Wrapper to make UnsafeCell sendable across threads
struct SyncUnsafeCell<T>(UnsafeCell<T>);
unsafe impl<T> Sync for SyncUnsafeCell<T> {}
impl<T> SyncUnsafeCell<T> {
    fn new(val: T) -> Self {
        SyncUnsafeCell(UnsafeCell::new(val))
    }
    fn get(&self) -> *mut T {
        self.0.get()
    }
}

#[no_mangle]
fn fence_sw_base() {
    let data = Arc::new(SyncUnsafeCell::new(0));
    let flag = Arc::new(AtomicBool::new(false));

    let th = {
        let (data, flag) = (data.clone(), flag.clone());
        thread::spawn(move || {
            unsafe { *data.get() = 42 };
            fence(Release);
            flag.store(true, Relaxed);
        })
    };

    if flag.load(Relaxed) {
        fence(Acquire);
        assert_eq!(42, unsafe { *data.get() });
    }
    th.join().unwrap();
}

#[no_mangle]
fn fence_sw_collapsed_store() {
    let data = Arc::new(SyncUnsafeCell::new(0));
    let flag = Arc::new(AtomicBool::new(false));

    let th = {
        let (data, flag) = (data.clone(), flag.clone());
        thread::spawn(move || {
            unsafe { *data.get() = 42 };
            flag.store(true, Release);
        })
    };

    if flag.load(Relaxed) {
        fence(Acquire);
        assert_eq!(42, unsafe { *data.get() });
    }
    th.join().unwrap();
}

#[no_mangle]
fn fence_sw_collapsed_load() {
    let data = Arc::new(SyncUnsafeCell::new(0));
    let flag = Arc::new(AtomicBool::new(false));

    let th = {
        let (data, flag) = (data.clone(), flag.clone());
        thread::spawn(move || {
            unsafe { *data.get() = 42 };
            fence(Release);
            flag.store(true, Relaxed);
        })
    };

    if flag.load(Acquire) {
        assert_eq!(42, unsafe { *data.get() });
    }
    th.join().unwrap();
}

#[no_mangle]
fn sb_fences() {
    let x = Arc::new(AtomicBool::new(false));
    let y = Arc::new(AtomicBool::new(false));

    let a = {
        let (x, y) = (x.clone(), y.clone());
        thread::spawn(move || {
            x.store(true, Relaxed);
            fence(SeqCst);
            y.load(Relaxed)
        })
    };

    y.store(true, Relaxed);
    fence(SeqCst);
    let b = x.load(Relaxed);

    if !a.join().unwrap() {
        assert!(b);
    }
}

#[no_mangle]
fn fence_hazard_pointer() {
    let reachable = Arc::new(AtomicBool::new(true));
    let protected = Arc::new(AtomicBool::new(false));
    let allocated = Arc::new(AtomicBool::new(true));

    let th = {
        let (reachable, protected, allocated) =
            (reachable.clone(), protected.clone(), allocated.clone());
        thread::spawn(move || {
            // put in protected list
            protected.store(true, Relaxed);
            fence(SeqCst);
            // validate, then access
            if reachable.load(Relaxed) {
                assert!(allocated.load(Relaxed));
            }
        })
    };

    // unlink/retire
    reachable.store(false, Relaxed);
    fence(SeqCst);
    // reclaim unprotected
    if !protected.load(Relaxed) {
        allocated.store(false, Relaxed);
    }

    th.join().unwrap();
}

#[no_mangle]
fn rwc_syncs() {
    #![allow(clippy::many_single_char_names)]
    let x = Arc::new(AtomicBool::new(false));
    let y = Arc::new(AtomicBool::new(false));

    let t2 = {
        let (x, y) = (x.clone(), y.clone());
        thread::spawn(move || {
            let a = x.load(Relaxed);
            fence(SeqCst);
            let b = y.load(Relaxed);
            (a, b)
        })
    };

    let t3 = {
        let x = x.clone();
        thread::spawn(move || {
            y.store(true, Relaxed);
            fence(SeqCst);
            x.load(Relaxed)
        })
    };

    x.store(true, Relaxed);

    let (a, b) = t2.join().unwrap();
    let c = t3.join().unwrap();

    if a && !b && !c {
        panic!();
    }
}

#[no_mangle]
fn w_rwc() {
    #![allow(clippy::many_single_char_names)]
    let x = Arc::new(AtomicBool::new(false));
    let y = Arc::new(AtomicBool::new(false));
    let z = Arc::new(AtomicBool::new(false));

    let t2 = {
        let (y, z) = (y.clone(), z.clone());
        thread::spawn(move || {
            let a = z.load(Acquire);
            fence(SeqCst);
            let b = y.load(Relaxed);
            (a, b)
        })
    };

    let t3 = {
        let x = x.clone();
        thread::spawn(move || {
            y.store(true, Relaxed);
            fence(SeqCst);
            x.load(Relaxed)
        })
    };

    x.store(true, Relaxed);
    z.store(true, Release);

    let (a, b) = t2.join().unwrap();
    let c = t3.join().unwrap();

    if a && !b && !c {
        panic!();
    }
}

fn main() {
    fence_sw_base();
    fence_sw_collapsed_store();
    fence_sw_collapsed_load();
    sb_fences();
    fence_hazard_pointer();
    rwc_syncs();
    w_rwc();
    println!("All tests completed");
}
