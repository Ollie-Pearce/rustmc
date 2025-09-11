#![feature(test)]

/// Benchmarks for the table. (example invocations in tests/tests_script.sh)
///
/// In practice you may want to run each one separately because running them all
/// at once seems to overtax the computer and slow down some of them (due to CPU
/// heating?).
///
/// Useful to also run this with tsan. In order to do that you need the rust src
/// code:
///
///     $ rustup component add --toolchain nightly rust-src
///
/// See historical benchmark results in "active_standby/benches/records/".
///
// 'test' is a special crate that requires extern crate even though we are using
// rust 2018.
// https://doc.rust-lang.org/nightly/edition-guide/rust-2018/module-system/path-clarity.html
extern crate test;
use active_standby::lockless::AsLockHandle;
use active_standby::UpdateTables;
use more_asserts::*;
use std::sync::Arc;
use parking_lot::RwLock;

struct AddOne {}
impl<'a> UpdateTables<'a, i32, ()> for AddOne {
    fn apply_first(&mut self, table: &'a mut i32) {
        *table += 1;
    }
    fn apply_second(mut self, table: &mut i32) {
        self.apply_first(table);
    }
}

struct SetZero {}
impl<'a> UpdateTables<'a, i32, ()> for SetZero {
    fn apply_first(&mut self, table: &mut i32) {
        *table = 0;
    }
    fn apply_second(mut self, table: &mut i32) {
        self.apply_first(table);
    }
}

pub mod lockless {
    active_standby::generate_lockless_aslockhandle!(i32);

    impl<'w> AsLockWriteGuard<'w> {
        pub fn add_one(&mut self) {
            self.guard.update_tables(super::AddOne {})
        }
        pub fn set_zero(&mut self) {
            self.guard.update_tables(super::SetZero {})
        }
    }
}

pub mod sync {
    active_standby::generate_sync_aslock!(i32);

    impl<'w> AsLockWriteGuard<'w> {
        pub fn add_one(&mut self) {
            self.guard.update_tables(super::AddOne {})
        }
        pub fn set_zero(&mut self) {
            self.guard.update_tables(super::SetZero {})
        }
    }
}

#[cfg(test)]
mod benchmarks {
    use super::*;

    // Test the speed of acquiring write guards when it never has to wait on
    // readers to release the table.
    #[bench]
    fn wguard_without_rcontention_lockless(b: &mut test::bench::Bencher) {
        let table = AsLockHandle::<i32>::from_identical(1, 1);
        b.iter(|| {
            let mut wg = table.write();
            wg.update_tables(AddOne {});
        });
    }
    #[bench]
    fn wguard_without_rcontention_sync(b: &mut test::bench::Bencher) {
        let table = Arc::new(sync::AsLock::new(1));
        b.iter(|| {
            let mut wg = table.write();
            wg.add_one();
        });
    }
    #[bench]
    fn wguard_without_rcontention_rwlock(b: &mut test::bench::Bencher) {
        let table = Arc::new(RwLock::new(1));
        b.iter(|| {
            let mut wg = table.write();
            *wg += 1;
        });
    }

    #[bench]
    fn wguard_rw_contention_lockless(b: &mut test::bench::Bencher) {
        let table = lockless::AsLockHandle::new(1);

        let _reader_handles: Vec<_> = (0..10)
            .map(|_| {
                let table = table.clone();
                std::thread::spawn(move || {
                    // Continually grab read guards.
                    while *table.read() != 0 {
                        // Hold the read guards to increase the chance of read
                        // 'contention'.
                        std::thread::sleep(std::time::Duration::from_micros(10));
                    }
                })
            })
            .collect();

        let _writer_handle = {
            let table = table.clone();
            std::thread::spawn(move || loop {
                let mut wg = table.write();
                wg.add_one();
            })
        };

        b.iter(|| {
            let mut wg = table.write();
            wg.add_one();
        });
    }
    #[bench]
    fn wguard_rw_contention_sync(b: &mut test::bench::Bencher) {
        let table = Arc::new(sync::AsLock::new(1));

        let _reader_handles: Vec<_> = (0..10)
            .map(|_| {
                let table = Arc::clone(&table);
                std::thread::spawn(move || {
                    // Continually grab read guards.
                    while *table.read() != 0 {
                        // Hold the read guards to increase the chance of read
                        // 'contention'.
                        std::thread::sleep(std::time::Duration::from_micros(10));
                    }
                })
            })
            .collect();

        let _writer_handle = {
            let table = Arc::clone(&table);
            std::thread::spawn(move || loop {
                let mut wg = table.write();
                wg.add_one();
            })
        };

        b.iter(|| {
            let mut wg = table.write();
            wg.add_one();
        });
    }
    #[bench]
    fn wguard_rw_contention_rwlock(b: &mut test::bench::Bencher) {
        let table = Arc::new(RwLock::new(1));

        let _reader_handles: Vec<_> = (0..10)
            .map(|_| {
                let table = Arc::clone(&table);
                std::thread::spawn(move || {
                    // Continually grab read guards.
                    while *table.read() != 0 {
                        // Hold the read guards to increase the chance of read
                        // 'contention'.
                        std::thread::sleep(std::time::Duration::from_micros(10));
                    }
                })
            })
            .collect();

        let _writer_handle = {
            let table = Arc::clone(&table);
            std::thread::spawn(move || loop {
                let mut wg = table.write();
                *wg += 1;
            })
        };

        b.iter(|| {
            let mut wg = table.write();
            *wg += 1;
        });
    }

    // Test the speed of acquiring the AsLockReadGuard when the writer never takes a guard
    // are there are no other readers.
    #[bench]
    fn rguard_no_contention_lockless(b: &mut test::bench::Bencher) {
        let table = AsLockHandle::<i32>::from_identical(1, 1);

        b.iter(|| {
            let rg = table.read();
            assert_eq!(*rg, 1);
        });
    }
    #[bench]
    fn rguard_no_contention_sync(b: &mut test::bench::Bencher) {
        let table = Arc::new(sync::AsLock::new(1));

        b.iter(|| {
            let rg = table.read();
            assert_eq!(*rg, 1);
        });
    }

    // The following section is the main thing we are interested in; how does
    // retreiving a read guard to the tables scale with lots of other readers
    // and an active writer. The tests compare 3 cases: lockless v. sync v.
    // plain RwLock.
    //
    // Tests for all cases are grouped together.
    // - num read threads: {1, 10, 20, 30}
    fn rguard_rw_contention_lockless(b: &mut test::bench::Bencher, num_readers: u32) {
        let table = lockless::AsLockHandle::from_identical(1, 1);

        let _reader_handles: Vec<_> = (0..num_readers)
            .map(|_| {
                let table = table.clone();
                std::thread::spawn(move || {
                    // Continually grab read guards.
                    while *table.read() != 0 {
                        // Hold the read guards to increase the change of read
                        // 'contention'.
                        std::thread::sleep(std::time::Duration::from_micros(100));
                    }
                })
            })
            .collect();

        let _writer_handles: Vec<_> = (0..2)
            .map(|_| {
                let table = table.clone();
                std::thread::spawn(move || loop {
                    let mut wg = table.write();
                    std::thread::sleep(std::time::Duration::from_micros(100));
                    wg.add_one();
                })
            })
            .collect();

        b.iter(|| {
            let rg = table.read();
            assert_gt!(*rg, 0);
        });
    }

    fn rguard_rw_contention_sync(b: &mut test::bench::Bencher, num_readers: u32) {
        let aslock = Arc::new(sync::AsLock::new(1));
        let _reader_handles: Vec<_> = (0..num_readers)
            .map(|_| {
                let aslock = Arc::clone(&aslock);
                std::thread::spawn(move || {
                    // Continually grab read guards.
                    while *aslock.read() != 0 {
                        // Hold the read guards to increase the change of read
                        // 'contention'.
                        std::thread::sleep(std::time::Duration::from_micros(100));
                    }
                })
            })
            .collect();
        let _writer_handles: Vec<_> = (0..2)
            .map(|_| {
                let aslock = Arc::clone(&aslock);
                std::thread::spawn(move || loop {
                    let mut wg = aslock.write();
                    std::thread::sleep(std::time::Duration::from_micros(100));
                    wg.add_one();
                })
            })
            .collect();

        b.iter(|| {
            let rg = aslock.read();
            assert_gt!(*rg, 0);
        });
    }

    fn rguard_rw_contention_rwlock(b: &mut test::bench::Bencher, num_readers: u32) {
        let table = Arc::new(RwLock::new(1));
        let _reader_handles: Vec<_> = (0..num_readers)
            .map(|_| {
                let table = Arc::clone(&table);
                std::thread::spawn(move || {
                    // Continually grab read guards.
                    while *table.read() != 0 {
                        // Hold the read guards to increase the change of read
                        // 'contention'.
                        std::thread::sleep(std::time::Duration::from_micros(100));
                    }
                })
            })
            .collect();

        let _writer_handles: Vec<_> = (0..2)
            .map(|_| {
                let table = Arc::clone(&table);
                std::thread::spawn(move || loop {
                    let mut wg = table.write();
                    std::thread::sleep(std::time::Duration::from_micros(100));
                    *wg += 1;
                })
            })
            .collect();

        b.iter(|| {
            let rg = table.read();
            assert_gt!(*rg, 0);
        });
    }

    #[bench]
    fn rguard_rw_contention_lockless_10(b: &mut test::bench::Bencher) {
        rguard_rw_contention_lockless(b, 10);
    }
    #[bench]
    fn rguard_rw_contention_lockless_20(b: &mut test::bench::Bencher) {
        rguard_rw_contention_lockless(b, 20);
    }
    #[bench]
    fn rguard_rw_contention_lockless_30(b: &mut test::bench::Bencher) {
        rguard_rw_contention_lockless(b, 30);
    }

    #[bench]
    fn rguard_rw_contention_sync_10(b: &mut test::bench::Bencher) {
        rguard_rw_contention_sync(b, 10);
    }
    #[bench]
    fn rguard_rw_contention_sync_20(b: &mut test::bench::Bencher) {
        rguard_rw_contention_sync(b, 20);
    }
    #[bench]
    fn rguard_rw_contention_sync_30(b: &mut test::bench::Bencher) {
        rguard_rw_contention_sync(b, 30);
    }

    #[bench]
    fn rguard_rw_contention_rwlock_10(b: &mut test::bench::Bencher) {
        rguard_rw_contention_rwlock(b, 10);
    }
    #[bench]
    fn rguard_rw_contention_rwlock_20(b: &mut test::bench::Bencher) {
        rguard_rw_contention_rwlock(b, 20);
    }
    #[bench]
    fn rguard_rw_contention_rwlock_30(b: &mut test::bench::Bencher) {
        rguard_rw_contention_rwlock(b, 30);
    }
}
