// If there are errors you need additional flags to use checkpointing (see
// docs). Example of what I did:
//
//      $ RUST_BACKTRACE=full RUSTFLAGS='--cfg loom' cargo +nightly test --test="loom" --features="loom/checkpoint" -- --nocapture

#[cfg(loom)]
#[cfg(test)]
mod loom_tests {
    use active_standby::lockless::AsLockHandle;
    use active_standby::sync::AsLock;
    use active_standby::UpdateTables;
    use loom::sync::{Arc, Condvar, LockResult, Mutex, MutexGuard};
    use loom::thread;

    struct AddOne {}
    impl<'a> UpdateTables<'a, i32, ()> for AddOne {
        fn apply_first(&mut self, table: &'a mut i32) {
            *table = *table + 1;
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

    // Wait as long as `condition` is still true.
    pub fn wait_while<'a, T, F>(
        cv: &Condvar,
        mut guard: MutexGuard<'a, T>,
        mut condition: F,
    ) -> LockResult<MutexGuard<'a, T>>
    where
        F: FnMut(&mut T) -> bool,
    {
        while condition(&mut *guard) {
            guard = cv.wait(guard)?;
        }
        Ok(guard)
    }

    #[test]
    fn lockless_single_thread() {
        loom::model(|| {
            let table = AsLockHandle::<i32>::from_identical(1, 1);
            {
                let mut wg = table.write();
                wg.update_tables(AddOne {});
            }

            let val;
            {
                let table = table.clone();
                val = thread::spawn(move || *table.read()).join().unwrap();
            }

            {
                let mut wg = table.write();
                wg.update_tables(AddOne {});
            }

            assert_eq!(val, 2);

            {
                let table = table.clone();
                let val = thread::spawn(move || *table.read()).join().unwrap();
                assert_eq!(val, 3);
            }
        });
    }

    #[test]
    fn sync_single_thread() {
        loom::model(|| {
            let table = Arc::new(AsLock::<i32>::new(1));
            {
                let mut wg = table.write();
                wg.update_tables(AddOne {});
            }

            let table2 = Arc::clone(&table);
            let val = thread::spawn(move || *table2.read()).join().unwrap();

            {
                let mut wg = table.write();
                wg.update_tables(AddOne {});
            }

            assert_eq!(val, 2);

            let table2 = Arc::clone(&table);
            let val = thread::spawn(move || *table2.read()).join().unwrap();
            assert_eq!(val, 3);
        });
    }

    #[test]
    fn lockless_multi_thread() {
        // Loom requires models to be deterministic. Without CondVar to
        // determine the actual order of execution, we can't promise the order
        // that spawned threads will run in. In this example, either the writer
        // or reader could run before the other. Loom catches this because the
        // epoch counter shows a different value when trying to claim a
        // AsLockWriteGuard. https://github.com/tokio-rs/loom/issues/233.
        loom::model(|| {
            let table = AsLockHandle::<i32>::from_identical(0, 0);

            let cond_cv = Arc::new((Mutex::new(0), Condvar::new()));
            let writer_handle = {
                let cond_cv = Arc::clone(&cond_cv);
                let table = table.clone();

                thread::spawn(move || {
                    let (cond, cv) = &*cond_cv;

                    let mut step_num;
                    {
                        let mut wg = table.write();
                        wg.update_tables(AddOne {});

                        *cond.lock().unwrap() += 1;
                        cv.notify_all();
                        step_num = wait_while(&cv, cond.lock().unwrap(), |step| *step < 2).unwrap();

                        // Write while the other thread holds the AsLockReadGuard.
                        wg.update_tables(AddOne {});

                        // Make sure to drop wg before notifying the reader.
                    }

                    *step_num += 1;
                    cv.notify_all();
                })
            };

            let (cond, cv) = &*cond_cv;
            {
                let rg;
                {
                    let mut step_num =
                        wait_while(&cv, cond.lock().unwrap(), |step| *step < 1).unwrap();

                    // Grab reader while holding the AsLockWriteGuard.
                    rg = table.read();
                    assert_eq!(*rg, 0);

                    *step_num += 1;
                    cv.notify_all();
                }
                let _step_num = wait_while(&cv, cond.lock().unwrap(), |step| *step < 3);
                // Retaining the old reader will retain the old value.
                assert_eq!(*rg, 0);
            }
            // Grabbing a new reader will show the newly published value.
            assert_eq!(*table.read(), 2);

            // Cannot join if there are any AsLockReadGuards alive in this thread
            // since this may deadlock.
            assert!(writer_handle.join().is_ok());
        });
    }

    #[test]
    fn sync_multi_thread() {
        loom::model(|| {
            let table = Arc::new(AsLock::<i32>::new(1));
            {
                let mut wg = table.write();
                wg.update_tables(AddOne {});
            }

            let table2 = Arc::clone(&table);
            let writer_handle = thread::spawn(move || {
                {
                    let mut wg = table2.write();
                    wg.update_tables(AddOne {});
                    wg.update_tables(AddOne {});
                }
                let mut wg = table2.write();
                wg.update_tables(SetZero {});
            });

            let table2 = Arc::clone(&table);
            let reader_handle = thread::spawn(move || {
                assert_eq!(*table2.read() % 2, 0);
            });

            assert_eq!(*table.read() % 2, 0);

            // Cannot join if there are any AsLockReadGuards alive in this thread
            // since this may deadlock.
            assert!(writer_handle.join().is_ok());
            assert!(reader_handle.join().is_ok());

            assert_eq!(*table.read(), 0);
        });
    }
}
