// Ported from loom unsafe_cell.rs
// GenMC-compatible version: replaces assert panics with detectable races


use std::cell::UnsafeCell;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::{Acquire, Release};
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

    fn trigger_error(&self) {
        unsafe {
            *self.cell.get() = 1;
        }
    }
}

#[no_mangle]
fn atomic_causality_success() {
    struct Chan {
        data: UnsafeCell<usize>,
        guard: AtomicUsize,
    }

    unsafe impl Sync for Chan {}

    impl Chan {
        fn set(&self) {
            unsafe {
                *self.data.get() += 123;
            }

            self.guard.store(1, Release);
        }

        fn get(&self) {
            if 0 == self.guard.load(Acquire) {
                return;
            }

            unsafe {
                let val = *self.data.get();
                if val != 123 {
                    let error = ErrorDetector::new();
                    error.trigger_error();
                }
            }
        }
    }

    let chan = Arc::new(Chan {
        data: UnsafeCell::new(0),
        guard: AtomicUsize::new(0),
    });

    let th = {
        let chan = chan.clone();
        thread::spawn(move || {
            chan.set();
        })
    };

    chan.get();
    th.join().unwrap();
    chan.get();
}

#[no_mangle]
fn atomic_causality_fail() {
    let error = Arc::new(ErrorDetector::new());

    struct Chan {
        data: UnsafeCell<usize>,
        guard: AtomicUsize,
    }

    unsafe impl Sync for Chan {}

    impl Chan {
        fn set(&self) {
            unsafe {
                *self.data.get() += 123;
            }

            self.guard.store(1, Release);
        }

        fn get(&self, error: &ErrorDetector) {
            unsafe {
                let val = *self.data.get();
                if val != 123 {
                    error.trigger_error();
                }
            }
        }
    }

    let chan = Arc::new(Chan {
        data: UnsafeCell::new(0),
        guard: AtomicUsize::new(0),
    });

    let th = {
        let chan = chan.clone();
        let error2 = error.clone();
        thread::spawn(move || {
            chan.set();
            error2.trigger_error();
        })
    };

    chan.get(&error);
    th.join().unwrap();
    chan.get(&error);
}

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

#[derive(Clone)]
struct Data(Arc<SyncUnsafeCell<usize>>);

impl Data {
    fn new(v: usize) -> Self {
        Data(Arc::new(SyncUnsafeCell::new(v)))
    }

    fn get(&self) -> usize {
        unsafe { *self.0.get() }
    }

    fn inc(&self) -> usize {
        unsafe {
            *self.0.get() += 1;
            *self.0.get()
        }
    }
}

#[no_mangle]
fn unsafe_cell_race_mut_mut_1() {
    let error = Arc::new(ErrorDetector::new());
    let x = Data::new(1);
    let y = x.clone();
    let error2 = error.clone();

    let th1 = thread::spawn(move || {
        x.inc();
        error2.trigger_error();
    });
    y.inc();
    error.trigger_error();

    th1.join().unwrap();

    if y.inc() != 4 {
        error.trigger_error();
    }
}

#[no_mangle]
fn unsafe_cell_race_mut_mut_2() {
    let error = Arc::new(ErrorDetector::new());
    let x = Data::new(1);
    let y = x.clone();
    let z = x.clone();
    let error2 = error.clone();
    let error3 = error.clone();

    let th1 = thread::spawn(move || {
        x.inc();
        error2.trigger_error();
    });
    let th2 = thread::spawn(move || {
        y.inc();
        error3.trigger_error();
    });

    th1.join().unwrap();
    th2.join().unwrap();

    if z.inc() != 4 {
        error.trigger_error();
    }
}

#[no_mangle]
fn unsafe_cell_race_mut_immut_1() {
    let error = Arc::new(ErrorDetector::new());
    let x = Data::new(1);
    let y = x.clone();
    let error2 = error.clone();

    let th1 = thread::spawn(move || {
        let val = x.inc();
        if val != 2 {
            error2.trigger_error();
        }
        error2.trigger_error();
    });
    y.get();
    error.trigger_error();

    th1.join().unwrap();

    if y.inc() != 3 {
        error.trigger_error();
    }
}

#[no_mangle]
fn unsafe_cell_race_mut_immut_2() {
    let error = Arc::new(ErrorDetector::new());
    let x = Data::new(1);
    let y = x.clone();
    let error2 = error.clone();

    let th1 = thread::spawn(move || {
        x.get();
        error2.trigger_error();
    });

    if y.inc() != 2 {
        error.trigger_error();
    }
    error.trigger_error();

    th1.join().unwrap();

    if y.inc() != 3 {
        error.trigger_error();
    }
}

#[no_mangle]
fn unsafe_cell_race_mut_immut_3() {
    let error = Arc::new(ErrorDetector::new());
    let x = Data::new(1);
    let y = x.clone();
    let z = x.clone();
    let error2 = error.clone();
    let error3 = error.clone();

    let th1 = thread::spawn(move || {
        let val = x.inc();
        if val != 2 {
            error2.trigger_error();
        }
        error2.trigger_error();
    });
    let th2 = thread::spawn(move || {
        y.get();
        error3.trigger_error();
    });

    th1.join().unwrap();
    th2.join().unwrap();

    if z.inc() != 3 {
        error.trigger_error();
    }
}

#[no_mangle]
fn unsafe_cell_race_mut_immut_4() {
    let error = Arc::new(ErrorDetector::new());
    let x = Data::new(1);
    let y = x.clone();
    let z = x.clone();
    let error2 = error.clone();
    let error3 = error.clone();

    let th1 = thread::spawn(move || {
        x.get();
        error2.trigger_error();
    });
    let th2 = thread::spawn(move || {
        let val = y.inc();
        if val != 2 {
            error3.trigger_error();
        }
        error3.trigger_error();
    });

    th1.join().unwrap();
    th2.join().unwrap();

    if z.inc() != 3 {
        error.trigger_error();
    }
}

#[no_mangle]
fn unsafe_cell_race_mut_immut_5() {
    let error = Arc::new(ErrorDetector::new());
    let x = Data::new(1);
    let y = x.clone();
    let z = x.clone();
    let error2 = error.clone();
    let error3 = error.clone();

    let th1 = thread::spawn(move || {
        x.get();
        error2.trigger_error();
    });
    let th2 = thread::spawn(move || {
        let val1 = y.get();
        if val1 != 1 {
            error3.trigger_error();
        }
        let val2 = y.inc();
        if val2 != 2 {
            error3.trigger_error();
        }
        error3.trigger_error();
    });

    th1.join().unwrap();
    th2.join().unwrap();

    if z.inc() != 3 {
        error.trigger_error();
    }
}

#[no_mangle]
fn unsafe_cell_ok_1() {
    let x = Data::new(1);

    if x.inc() != 2 {
        let error = ErrorDetector::new();
        error.trigger_error();
    }

    let th1 = thread::spawn(move || {
        if x.inc() != 3 {
            let error = ErrorDetector::new();
            error.trigger_error();
        }
        x
    });

    let x = th1.join().unwrap();

    if x.inc() != 4 {
        let error = ErrorDetector::new();
        error.trigger_error();
    }
}

#[no_mangle]
fn unsafe_cell_ok_2() {
    let x = Data::new(1);

    if x.get() != 1 {
        let error = ErrorDetector::new();
        error.trigger_error();
    }
    if x.inc() != 2 {
        let error = ErrorDetector::new();
        error.trigger_error();
    }

    let th1 = thread::spawn(move || {
        if x.get() != 2 {
            let error = ErrorDetector::new();
            error.trigger_error();
        }
        if x.inc() != 3 {
            let error = ErrorDetector::new();
            error.trigger_error();
        }
        x
    });

    let x = th1.join().unwrap();

    if x.get() != 3 {
        let error = ErrorDetector::new();
        error.trigger_error();
    }
    if x.inc() != 4 {
        let error = ErrorDetector::new();
        error.trigger_error();
    }
}

#[no_mangle]
fn unsafe_cell_ok_3() {
    let x = Data::new(1);
    let y = x.clone();

    let th1 = thread::spawn(move || {
        if x.get() != 1 {
            let error = ErrorDetector::new();
            error.trigger_error();
        }

        let z = x.clone();
        let th2 = thread::spawn(move || {
            if z.get() != 1 {
                let error = ErrorDetector::new();
                error.trigger_error();
            }
        });

        if x.get() != 1 {
            let error = ErrorDetector::new();
            error.trigger_error();
        }
        th2.join().unwrap();
    });

    if y.get() != 1 {
        let error = ErrorDetector::new();
        error.trigger_error();
    }

    th1.join().unwrap();

    if y.inc() != 2 {
        let error = ErrorDetector::new();
        error.trigger_error();
    }
}

#[no_mangle]
fn unsafe_cell_access_after_sync() {
    let error = Arc::new(ErrorDetector::new());
    let s1 = Arc::new((AtomicUsize::new(0), SyncUnsafeCell::new(0)));
    let s2 = s1.clone();
    let error2 = error.clone();

    thread::spawn(move || {
        s1.0.store(1, Release);
        unsafe { *s1.1.get() = 1 };
        error2.trigger_error();
    });

    if 1 == s2.0.load(Acquire) {
        unsafe { *s2.1.get() = 2 };
        error.trigger_error();
    }
}

fn main() {
    atomic_causality_success();
    atomic_causality_fail();
    unsafe_cell_race_mut_mut_1();
    unsafe_cell_race_mut_mut_2();
    unsafe_cell_race_mut_immut_1();
    unsafe_cell_race_mut_immut_2();
    unsafe_cell_race_mut_immut_3();
    unsafe_cell_race_mut_immut_4();
    unsafe_cell_race_mut_immut_5();
    unsafe_cell_ok_1();
    unsafe_cell_ok_2();
    unsafe_cell_ok_3();
    unsafe_cell_access_after_sync();
    println!("All tests completed");
}
