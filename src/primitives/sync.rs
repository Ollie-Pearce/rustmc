use crate::types::*;
use std::fmt;
use std::mem::ManuallyDrop;

// When `update_tables` is called, the `standby_table` is updated immediately.
// We then store the update ops to be replayed on the other table.
type OpsToReplay<T> = Mutex<Vec<Box<dyn FnOnce(&mut T) + Send>>>;
type OpsToReplayGuard<'w, T> = MutexGuard<'w, Vec<Box<dyn FnOnce(&mut T) + Send>>>;

/// Struct for holding tables that can be interacted with like an RwLock,
/// including being shared across threads/tasks via Arc (as opposed to the
/// lockless version which requires independent copies per thread/task).
pub struct AsLock<T> {
    // The underlying tables. These tables will be utilized directly both for
    // writing and reading. The RwLock guarantees that this will be safe; in
    // practice blocking writes when there are pre-existing read guards. The use
    // of AtomicPtr allows this struct to be `Sync`, so that when a `AsLockWriteGuard`
    // is dropped, the tables can be swapped without ever blocking reads.
    active_table: AtomicPtr<RwLock<T>>,
    standby_table: AtomicPtr<RwLock<T>>,

    /// Log of operations to be performed on the second table. This gets played
    /// on the standby table when creating an AsLockWriteGuard, as opposed to when
    /// dropping it, to minimize lock contention. This is in the hopes that by
    /// waiting until the next time a `AsLockWriteGuard` is created, we give the
    /// readers time to switch to reading from the new `active_table`.
    ///
    /// This mutex is used to guarantee that `write` is single threaded, and so
    /// locking it must be done before any operation other that `read`.
    ops_to_replay: OpsToReplay<T>,
}

/// Guard used for updating the tables.
pub struct AsLockWriteGuard<'w, T> {
    // Pointers to the tables. Used to swap them on drop.
    active_table: &'w AtomicPtr<RwLock<T>>,
    standby_table: &'w AtomicPtr<RwLock<T>>,

    // Guard used to mutably access `standby_table` without constantly having to
    // load the atomic pointer and write lock the table. This is wrapped inside
    // of `ManuallyDrop` to guarantee that the table is unlocked before swapping
    // the active and standby tables. This is needed to gurantee that reads are
    // never blocked.
    //
    // This introduces the risk of a deadlock though if we forget to release
    // `guard`.
    guard: ManuallyDrop<RwLockWriteGuard<'w, T>>,

    // Hold onto updates for replay when the next AsLockWriteGuard is created. This
    // Mutex also prevents any other thread from utilizing the `AsLock`, other
    // than calls to `read`.
    ops_to_replay: OpsToReplayGuard<'w, T>,
}

// Define AsLockReadGuard locally so that the type names are consistent; across
// lockless & sync, as well as internally (AsLockWriteGuard & RwLockAsLockReadGuard
// seem unwieldy).
pub type AsLockReadGuard<'r, T> = RwLockReadGuard<'r, T>;

impl<T> AsLock<T> {
    /// Create an `AsLock`. t1 & t2 must be identical; this is left to the
    /// caller to enforce.
    pub fn from_identical(t1: T, t2: T) -> AsLock<T> {
        AsLock {
            active_table: AtomicPtr::new(Box::into_raw(Box::new(RwLock::new(t1)))),
            standby_table: AtomicPtr::new(Box::into_raw(Box::new(RwLock::new(t2)))),
            ops_to_replay: Mutex::default(),
        }
    }

    pub fn read(&self) -> AsLockReadGuard<'_, T> {
        // SAFETY: The safety issue here is active_table being an invalid ptr.
        // This should never happen since standby/active table are created on
        // creation and only dropped when AsLock is dropped. In between they are
        // swapped, but that shouldn't affect their valididty as pointers.
        unsafe { &*self.active_table.load(Ordering::SeqCst) }.read()
    }

    /// Create an AsLockWriteGuard to allow users to update the the data. There will
    /// only be 1 AsLockWriteGuard at a time.
    ///
    /// This function may be slow because:
    /// 1. Another AsLockWriteGuard exists. In practice this means lock contention on
    ///    `ops_to_replay`.
    /// 2. A AsLockReadGuard still points to the standby table, meaning that this
    ///    AsLockReadGuard came into existence before the last AsLockWriteGuard was dropped.
    /// 3. Replaying all of the updates that were applied to the last
    ///    AsLockWriteGuard.
    pub fn write(&self) -> AsLockWriteGuard<'_, T> {
        // Done first to ensure that writes are single threaded.
        let mut ops_to_replay = self.ops_to_replay.lock();

        // Grab the standby table and obtain a `AsLockWriteGuard` to it. This may hang
        // on `AsLockReadGuard`s which exist from before the last swap.
        //
        // SAFETY: The safety issue here is standby_table being an invalid ptr.
        // This should never happen since standby/active table are created on
        // creation and only dropped when `AsLock` is dropped. In between they
        // are swapped, but that shouldn't affect their valididty as pointers.
        let mut wg = unsafe { &*self.standby_table.load(Ordering::SeqCst) }.write();

        // Replay all ops on the standby table.
        for op in ops_to_replay.drain(..) {
            op(&mut wg);
        }
        ops_to_replay.clear();

        AsLockWriteGuard {
            guard: ManuallyDrop::new(wg),
            active_table: &self.active_table,
            standby_table: &self.standby_table,
            ops_to_replay,
        }
    }
}

impl<T> Drop for AsLock<T> {
    fn drop(&mut self) {
        // SAFETY: Tables are created on class creation, and while swapped, they
        // are never changed to an invalid state during the life of `AsLock`.
        unsafe {
            let _active_table = Box::from_raw(self.active_table.load(Ordering::SeqCst));
            let _standby_table = Box::from_raw(self.standby_table.load(Ordering::SeqCst));
        }
    }
}

impl<T> AsLock<T>
where
    T: Clone,
{
    pub fn new(t: T) -> AsLock<T> {
        Self::from_identical(t.clone(), t)
    }
}

impl<T> Default for AsLock<T>
where
    T: Default,
{
    fn default() -> AsLock<T> {
        Self::from_identical(T::default(), T::default())
    }
}

impl<T: fmt::Debug> fmt::Debug for AsLock<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let num_ops_to_replay = self.ops_to_replay.lock().len();
        f.debug_struct("AsLock")
            .field("num_ops_to_replay", &num_ops_to_replay)
            .field("standby_table", &*self.write())
            .field("active_table", &*self.read())
            .finish()
    }
}

impl<'w, T> AsLockWriteGuard<'w, T> {
    /// Takes an update which will change the state of the underlying data. This
    /// is done through the interface of UpdateTables.
    ///
    /// Users should never use the return value to directly mutate the tables,
    /// since this will lead to them going out of sync.
    ///
    /// The update passed in must be valid for 'static because it will outlive
    /// the AsLockWriteGuard taking the update, so we can't make any limitations on
    /// it.
    pub fn update_tables<'a, R>(
        &'a mut self,
        mut update: impl UpdateTables<'a, T, R> + 'static + Sized + Send,
    ) -> R {
        let res = update.apply_first(&mut self.guard);

        self.ops_to_replay.push(Box::new(move |table| {
            update.apply_second(table);
        }));

        res
    }

    /// Like `update_tables` but allows the user to pass a closure for
    /// convenience. Only allows return values that own their data.
    ///
    /// TODO: Consider allowing return values that have lifetimes, this should
    /// likely be as safe as the explicit UpdateTables trait.
    pub fn update_tables_closure<R>(
        &mut self,
        update: impl Fn(&mut T) -> R + 'static + Sized + Send,
    ) -> R {
        let res = update(&mut self.guard);

        self.ops_to_replay.push(Box::new(move |table| {
            update(table);
        }));

        res
    }
}

impl<'w, T> Drop for AsLockWriteGuard<'w, T> {
    fn drop(&mut self) {
        // SAFETY: We must guarantee that all calls to AsLockWriteGuard::drop drop
        // self.guard to unlock the table.
        //
        // SAFETY: We can guarantee that the guard is valid (and therefore safe
        // to drop), because the value is only ever set on AsLockWriteGuard creation
        // from a valid value and is never changed.
        unsafe { ManuallyDrop::drop(&mut self.guard) };

        // Swap the tables after releasing the RwLockAsLockWriteGuard to guarantee
        // reads are never blocked.
        fence(Ordering::SeqCst);

        let active_table = self.active_table.load(Ordering::SeqCst);
        let standby_table = self.standby_table.load(Ordering::SeqCst);
        assert_ne!(active_table, standby_table);

        // Swap the active and standby tables. These should never fail because
        // there can only ever be 1 writer which spawns only 1 AsLockWriteGuard.
        let res = self.active_table.compare_exchange(
            active_table,
            standby_table,
            Ordering::SeqCst,
            Ordering::SeqCst,
        );
        assert_eq!(res, Ok(active_table));

        let res = self.standby_table.compare_exchange(
            standby_table,
            active_table,
            Ordering::SeqCst,
            Ordering::SeqCst,
        );
        assert_eq!(res, Ok(standby_table));

        // Only after swapping the tables should we drop the Mutex to
        // `ops_to_replay`, allowing a new AsLockWriteGuard.
    }
}

impl<'w, T> std::ops::Deref for AsLockWriteGuard<'w, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &**self.guard
    }
}

impl<'w, T: fmt::Debug> fmt::Debug for AsLockWriteGuard<'w, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use std::ops::Deref;
        f.debug_struct("AsLockWriteGuard")
            .field("num_ops_to_replay", &self.ops_to_replay.len())
            .field("standby_table", self.deref())
            .finish()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::thread;

    struct PushVec<T> {
        value: T,
    }
    impl<'a, T> UpdateTables<'a, Vec<T>, ()> for PushVec<T>
    where
        T: Clone,
    {
        fn apply_first(&mut self, table: &'a mut Vec<T>) {
            table.push(self.value.clone());
        }
        fn apply_second(self, table: &mut Vec<T>) {
            table.push(self.value); // Move the value instead of cloning.
        }
    }

    struct PopVec {}
    impl PopVec {
        fn apply<T>(&mut self, table: &mut Vec<T>) -> Option<T> {
            table.pop()
        }
    }
    impl<'a, T> UpdateTables<'a, Vec<T>, Option<T>> for PopVec {
        fn apply_first(&mut self, table: &'a mut Vec<T>) -> Option<T> {
            self.apply(table)
        }
        fn apply_second(mut self, table: &mut Vec<T>) {
            (&mut self).apply(table);
        }
    }

    /// This is an example of what not to do!
    struct MutableRef {}
    impl<'a, T> UpdateTables<'a, Vec<T>, &'a mut T> for MutableRef {
        fn apply_first(&mut self, table: &'a mut Vec<T>) -> &'a mut T {
            &mut table[0]
        }
        fn apply_second(self, table: &mut Vec<T>) {
            let _ = &mut table[0];
        }
    }

    #[test]
    fn one_write_guard() {
        // TODO: Have a multithreaded test for this.
        let writer = AsLock::<Vec<i32>>::default();
        let _wg = writer.write();
        // let wg2 = writer.write();
    }

    #[test]
    fn publish_update() {
        let aslock = Arc::new(AsLock::<Vec<i32>>::default());
        assert_eq!(aslock.read().len(), 0);

        {
            let mut wg = aslock.write();
            wg.update_tables(PushVec { value: 2 });
            assert_eq!(wg.len(), 1);
            {
                // Perform check in another thread to avoid potential deadlock
                // (calling both read and write on aslock at the same time).
                let aslock = Arc::clone(&aslock);
                assert!(thread::spawn(move || {
                    assert_eq!(aslock.read().len(), 0);
                })
                .join()
                .is_ok());
            }
        }

        // When the write guard is dropped it publishes the changes to the readers.
        assert_eq!(*aslock.read(), vec![2]);
    }

    #[test]
    fn update_tables_closure() {
        let aslock = Arc::new(AsLock::<Vec<i32>>::default());
        assert_eq!(aslock.read().len(), 0);

        {
            let mut wg = aslock.write();
            wg.update_tables_closure(|vec| vec.push(2));
            assert_eq!(wg.len(), 1);
            {
                // Perform check in another thread to avoid potential deadlock
                // (calling both read and write on aslock at the same time).
                let aslock = Arc::clone(&aslock);
                assert!(thread::spawn(move || {
                    assert_eq!(aslock.read().len(), 0);
                })
                .join()
                .is_ok());
            }
        }

        // When the write guard is dropped it publishes the changes to the readers.
        assert_eq!(*aslock.read(), vec![2]);
    }

    #[test]
    fn multi_apply() {
        let aslock = AsLock::<Vec<i32>>::default();
        {
            let mut wg = aslock.write();
            wg.update_tables(PushVec { value: 2 });
            wg.update_tables(PushVec { value: 3 });
            wg.update_tables(PushVec { value: 4 });
            wg.update_tables(PopVec {});
            wg.update_tables(PushVec { value: 5 });
        }
        assert_eq!(*aslock.read(), vec![2, 3, 5]);
    }

    #[test]
    fn multi_publish() {
        let aslock = AsLock::<Vec<Box<i32>>>::default();
        {
            let mut wg = aslock.write();
            wg.update_tables(PushVec { value: Box::new(2) });
            wg.update_tables(PushVec { value: Box::new(3) });
            wg.update_tables(PopVec {});
            wg.update_tables(PushVec { value: Box::new(5) });
        }
        assert_eq!(*aslock.read(), vec![Box::new(2), Box::new(5)]);

        {
            let mut wg = aslock.write();
            wg.update_tables(PushVec { value: Box::new(9) });
            wg.update_tables(PushVec { value: Box::new(8) });
            wg.update_tables(PopVec {});
            wg.update_tables(PushVec { value: Box::new(7) });
        }
        assert_eq!(
            *aslock.read(),
            vec![Box::new(2), Box::new(5), Box::new(9), Box::new(7)]
        );

        {
            let mut wg = aslock.write();
            wg.update_tables(PopVec {});
        }
        assert_eq!(*aslock.read(), vec![Box::new(2), Box::new(5), Box::new(9)]);
    }

    #[test]
    fn multi_thread() {
        let aslock = Arc::new(AsLock::<Vec<i32>>::default());
        let aslock2 = Arc::clone(&aslock);
        let handler = thread::spawn(move || {
            while *aslock2.read() != vec![2, 3, 5] {
                // Since commits oly happen when an AsLockWriteGuard is dropped
                // no reader will see this state.
                assert_ne!(*aslock2.read(), vec![2, 3, 4]);
            }

            // Show multiple readers in multiple threads.
            let aslock3 = Arc::clone(&aslock2);
            let handler = thread::spawn(move || while *aslock3.read() != vec![2, 3, 5] {});
            assert!(handler.join().is_ok());
        });

        {
            let mut wg = aslock.write();
            wg.update_tables(PushVec { value: 2 });
            wg.update_tables(PushVec { value: 3 });
            wg.update_tables(PushVec { value: 4 });
            wg.update_tables(PopVec {});
            wg.update_tables(PushVec { value: 5 });
        }

        assert!(handler.join().is_ok());
    }

    #[test]
    fn debug_str() {
        let aslock = AsLock::<Vec<i32>>::default();
        assert_eq!(
            format!("{:?}", aslock),
            "AsLock { num_ops_to_replay: 0, standby_table: [], active_table: [] }"
        );
        {
            let mut wg = aslock.write();
            wg.update_tables(PushVec { value: 2 });
            assert_eq!(
                format!("{:?}", wg),
                "AsLockWriteGuard { num_ops_to_replay: 1, standby_table: [2] }"
            );
        }
        assert_eq!(
            format!("{:?}", aslock),
            "AsLock { num_ops_to_replay: 1, standby_table: [2], active_table: [2] }"
        );
        // The aliased sync lock shows up in this debug. What we mostly care
        // about is that this says AsLockReadGuard and shows the underlying data. It's
        // fine to update this if we ever change the underlying RwLock.
        assert_eq!(format!("{:?}", aslock.read()), "[2]");
    }
}
