/// This contains the primitives for building the lockless interface for
/// active_standby. The public interface is made up of:
/// - AsLockHandle - analagous to Arc<RwLock>.
/// - AsLockWriteGuard - analagous to RwLockAsLockWriteGuard.
/// - AsLockReadGuard - analagous to RwLockReadGuard.
///
/// A high level outline of how this is achieved and the required invariants to
/// guarantee safety:
///
/// To avoid the need for expensive synchronization, and guarantee the reader is
/// never blocked each reader will contain:
/// 1. An epoch counter, which is used for synchronization. This prevents Reader
///    from being Sync, requiring each task/thread to have its own Reader.
/// 2. A pointer to the active_table which is always usable.
///
/// This simplicity pushes the synchronization complexity onto the Writer, who
/// must handle the following responsibilities:
/// 1. Updating both tables. This involves mutating the standby table as updates
///    come in and storing updates to replay on the other table.
/// 2. Swapping the tables. This involves iterating over all readers and
///    updating the table they point to.
/// 3. The Writer must then wait to perform more updates until the new standby
///    table is free. The Readers only synchronize by incrementing their epoch
///    counter to allow the Writer to act properly.
/// 4. Create new Readers.
///
/// In order to wrap these together we expose the AsLockHandle, which is Send,
/// but not Sync, since each one holds its own reader. AsLockHandle must
/// guarantee that calls to Writer, which is shared between are tasks/threads,
/// are single threaded, as Writer does update the systems state. AsLockHandle
/// must also guarantee that Writer outlives all Readers since Writer owns the
/// tables.
use crate::types::*;
use slab::Slab;
use std::fmt;

struct TableAndEpoch<T> {
    table: AtomicPtr<T>,
    epoch: AtomicUsize,
}

/// The shared state of all Readers. Used to synchronize between Readers and the
/// Writer.
type ReadersList<T> = Arc<Mutex<Slab<Arc<TableAndEpoch<T>>>>>;

/// Interface used to gain non-blocking read access to one of the tables. One
/// per thread/task, not meant to be sync.
struct Reader<T> {
    // This readers state. Put behind an Arc since it is shared with the Writer
    // which must update the table on swap and read the epoch to synchronize.
    sync_state: Arc<TableAndEpoch<T>>,

    // Used to remove self from `readers` on drop.
    key_in_readers: usize,

    // List of all readers, used on Drop.
    readers: ReadersList<T>,
}

/// Guard used for obtaining const access to the active table.
pub struct AsLockReadGuard<'r, T> {
    // Read by callers when dereferenceing the table.
    active_table: &'r T,

    // Incremented on Drop.
    epoch: &'r AtomicUsize,
}

/// Interface for mutating the state of the system, primarily for updating the
/// tables.
struct Writer<T> {
    // The 2 tables. Writer owns the tables and so must outlive all Readers.
    // These are created on Writer construction, and while they are swapped,
    // they always point only to the 2 tables initially passed in, meaning they
    // remain valid pointers until Writer is dropped.
    active_table: Box<T>,
    standby_table: Box<T>,

    // Log of operations to be performed on the second table.
    //
    // During an AsLockWriteGuard's lifetime, it mutates the standby table, but leaves
    // the active one constant for reads. These tables are then swapped when
    // the AsLockWriteGuard is dropped. Therefore, the next time an AsLockWriteGuard is
    // created, the standby table it points to will still need to have these
    // updates applied to it to keep the tables sychronized.
    ops_to_replay: Vec<Box<dyn FnOnce(&mut T) + Send>>,

    // List of all readers. Used for:
    // - Creating new readers.
    // - Blocking AsLockWriteGuard creation due to existing reads.
    // - Updating the active_table on swap.
    readers: ReadersList<T>,

    // A record of readers and their epoch after the most recent swap.
    //
    // Filled by the AsLockWriteGuard when it is dropped, and used by the Writer to
    // block creation of a new AsLockWriteGuard until there are no AsLockReadGuards left
    // pointing to the standby table.
    //
    // {reader_key : first_epoch_after_swap}.
    blocking_readers: std::collections::HashMap<usize, usize>,
}

/// Public primitive for building lockess active_standby data structures. Give
/// users both read and write access to the tables.
///
/// It is worth noting that this data structure should not be shared across
/// threads/tasks. Rather think of it as closer to a parallel of Arc<RwLock>
/// than a plain RwLock. Meaning that each thread/task should get its own
/// AsLockHandle (via clone).
pub struct AsLockHandle<T> {
    writer: Arc<Mutex<Writer<T>>>,
    reader: Reader<T>,

    // Make un-sync.
    _not_sync: std::cell::UnsafeCell<fn(&T)>,
}

/// Interface for updating the tables. Produced by the AsLockHandle, not the
/// Writer.
pub struct AsLockWriteGuard<'w, T> {
    writer: MutexGuard<'w, Writer<T>>,
}

impl<T> Reader<T> {
    /// Obtain a read guard with which to inspect the active table.
    ///
    /// This should never block free since there is nothing to lock, and the
    /// Writer is responsible for never mutating the table that an AsLockReadGuard
    /// points to.
    ///
    /// The steps involved are:
    /// 1. Arc dereference to shared state.
    /// 2. AtomicUsize increment to lock the table.
    /// 3. AtomicPtr load to the table.
    pub fn read(&self) -> AsLockReadGuard<'_, T> {
        // 1. Load the shared state.
        let TableAndEpoch { table, epoch } = &*self.sync_state;

        // 2. Lock the active table.
        let old_epoch = epoch.load(Ordering::Acquire);
        assert_eq!(old_epoch % 2, 0, "Reader is not reentrant");

        // The reader must update the epoch before taking the table. This
        // effectively locks the active_table, making it safe for the reader to
        // proceed knowing that the Writer will not be able to access this table
        // until epoch is incremented again.
        epoch.store(old_epoch + 1, Ordering::Release);
        fence(Ordering::SeqCst);

        // 3. Atomic load of the active table. The actual dereference will
        //    happen when the user makes use the the AsLockReadGuard.
        //
        // SAFETY: Memory safety (valid pointer) is guaranteed by
        // AsLockHandle/Writer, which enforce that the tables are created before
        // any Reader exists and dropped only after all readers. Further the
        // tables themselves are never moved in memory.
        //
        // SAFETY: Thread safety is what must be handled by us manually. The
        // `epoch` counter by the Reader and `await_standby_table_free` by the
        // Writer.
        let active_table = unsafe { &*table.load(Ordering::SeqCst) };
        AsLockReadGuard {
            active_table,
            epoch,
        }
    }
}

impl<T> Drop for Reader<T> {
    /// Remove the reader from the shared state list.
    fn drop(&mut self) {
        self.readers.lock().remove(self.key_in_readers);
    }
}

impl<T: fmt::Debug> fmt::Debug for Reader<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Reader")
            .field("num_readers", &self.readers.lock().len())
            .field("active_table", &*self.read())
            .finish()
    }
}

impl<'r, T> Drop for AsLockReadGuard<'r, T> {
    /// Update the epoch counter to notify the Writer that we are done using the
    /// active table and so it is available for use as the new standby table.
    fn drop(&mut self) {
        let old_epoch = self.epoch.load(Ordering::Acquire);
        debug_assert_eq!(old_epoch % 2, 1);
        self.epoch.store(old_epoch + 1, Ordering::Release);
    }
}

impl<'r, T> std::ops::Deref for AsLockReadGuard<'r, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.active_table
    }
}

impl<'r, T: fmt::Debug> fmt::Debug for AsLockReadGuard<'r, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.active_table.fmt(f)
    }
}

impl<T> Writer<T> {
    /// Create a `Writer` which will be the owner of the active and standby
    /// tables. t1 & t2 must be identical; this is left to the caller to
    /// enforce.
    pub fn from_identical(t1: T, t2: T) -> Writer<T> {
        Writer {
            active_table: Box::new(t1),
            standby_table: Box::new(t2),
            ops_to_replay: vec![],
            readers: Arc::new(Mutex::new(Slab::with_capacity(1024))),
            blocking_readers: std::collections::HashMap::new(),
        }
    }

    /// Creates a new `Reader`.
    ///
    /// Note that `Reader` creation is somewhat delicate since if it races with
    /// a swap, we may have a `Reader` pointing to the standby_table. This is
    /// covered though since Writer single threaded (enforced by AsLockHandle).
    pub fn new_reader(&mut self) -> Reader<T> {
        let readers = Arc::clone(&self.readers);

        let sync_state = Arc::new(TableAndEpoch {
            table: AtomicPtr::new(self.active_table.as_mut() as *mut T),
            epoch: AtomicUsize::new(0),
        });
        let key_in_readers = readers.lock().insert(Arc::clone(&sync_state));

        Reader {
            sync_state,
            key_in_readers,
            readers,
        }
    }

    /// Hangs until the standby table is free of `AsLockReadGuards` which point to it.
    /// This means that the Writer can produce an AsLockWriteGuard to it and perform
    /// updates.
    fn await_standby_table_free(&mut self) {
        while !self.blocking_readers.is_empty() {
            let readers = self.readers.lock();
            self.blocking_readers.retain(|key, first_epoch_after_swap| {
                let epoch = match readers.get(*key) {
                    None => {
                        // This Reader has been dropped.
                        return false;
                    }
                    Some(table_and_epoch) => table_and_epoch.epoch.load(Ordering::Acquire),
                };

                epoch <= *first_epoch_after_swap && *first_epoch_after_swap % 2 != 0
            });

            if !self.blocking_readers.is_empty() {
                // Instead of just busy looping we will (potentially) yield this
                // thread and come back when the OS returns to us.
                spin_loop();
            }
        }
    }
}

impl<T> std::fmt::Debug for Writer<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Writer")
            .field("num_readers", &self.readers.lock().len())
            .field("ops_to_replay", &self.ops_to_replay.len())
            .field("standby_table", &self.standby_table)
            .finish()
    }
}

impl<T> AsLockHandle<T> {
    /// Create an `AsLockHandle`. t1 & t2 must be identical; this is left to the
    /// caller to enforce.
    pub fn from_identical(t1: T, t2: T) -> AsLockHandle<T> {
        let mut writer = Writer::from_identical(t1, t2);
        let reader = writer.new_reader();

        AsLockHandle {
            writer: Arc::new(Mutex::new(writer)),
            reader,
            _not_sync: std::cell::UnsafeCell::new(|_| {}),
        }
    }

    /// Obtain a read guard with which to inspect the active table.
    ///
    /// This is wait free since there is nothing to lock, and the Writer is
    /// responsible for never mutating the table that an AsLockReadGuard points to.
    pub fn read(&self) -> AsLockReadGuard<'_, T> {
        self.reader.read()
    }

    /// Create a `AsLockWriteGuard` which is used to update the underlying tables.
    ///
    /// This function may be slow because:
    /// 1. Another AsLockWriteGuard exists. In practice this means lock contention on
    ///    `writer`.
    /// 2. A `AsLockReadGuard` still points to the standby table, meaning that this
    ///    `AsLockReadGuard` came into existence before the last `AsLockWriteGuard` was
    ///    dropped.
    /// 3. Replaying all of the updates that were applied to the last
    ///    `AsLockWriteGuard`.
    pub fn write(&self) -> AsLockWriteGuard<'_, T> {
        let mut mg = self.writer.lock();
        // Explicitly cast MutexGuard into Writer in order for split borrowing
        // to work. Without this line the compiler thinks that the borrow of
        // standby_table and ops_to_replay are conflicting mutable borrows
        // https://doc.rust-lang.org/nomicon/borrow-splitting.html
        let writer: &mut Writer<_> = &mut mg;

        // Wait until the standby table is free of AsLockReadGuards so it is safe to
        // update.
        writer.await_standby_table_free();
        std::sync::atomic::compiler_fence(Ordering::SeqCst);

        // Bring the standby table up to date, it should now match the active
        // table.
        for op in writer.ops_to_replay.drain(..) {
            op(&mut writer.standby_table);
        }
        writer.ops_to_replay.clear();

        AsLockWriteGuard { writer: mg }
    }
}

impl<T> AsLockHandle<T>
where
    T: Clone,
{
    pub fn new(t: T) -> AsLockHandle<T> {
        Self::from_identical(t.clone(), t)
    }
}

impl<T> Default for AsLockHandle<T>
where
    T: Default,
{
    fn default() -> AsLockHandle<T> {
        Self::from_identical(T::default(), T::default())
    }
}

impl<T> Clone for AsLockHandle<T> {
    fn clone(&self) -> AsLockHandle<T> {
        let writer = Arc::clone(&self.writer);
        let reader = writer.lock().new_reader();
        AsLockHandle {
            writer,
            reader,
            _not_sync: std::cell::UnsafeCell::new(|_| {}),
        }
    }
}

impl<T> std::fmt::Debug for AsLockHandle<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let writer = self.writer.lock();
        let num_readers = writer.readers.lock().len();
        let num_ops_to_replay = writer.ops_to_replay.len();
        f.debug_struct("AsLockHandle")
            .field("num_readers", &num_readers)
            .field("num_ops_to_replay", &num_ops_to_replay)
            // No nead to `await_standby_table_free` since this is a read, so
            // doesn't interfere with other readers, and is under the write
            // lock, so protected from data races.
            .field("standby_table", &writer.standby_table)
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
        // Explicitly cast MutexGuard into Writer in order for split borrowing
        // to work. Without this line the compiler thinks that the borrow of
        // standby_table and ops_to_replay are conflicting mutable borrows
        // https://doc.rust-lang.org/nomicon/borrow-splitting.html
        let writer: &mut Writer<_> = &mut self.writer;

        let res = update.apply_first(&mut writer.standby_table);

        writer.ops_to_replay.push(Box::new(move |table| {
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
        // See comments on `Table::standby_table_mut` for safety.
        let res = update(&mut self.writer.standby_table);

        self.writer.ops_to_replay.push(Box::new(move |table| {
            update(table);
        }));

        res
    }
}

impl<'w, T> Drop for AsLockWriteGuard<'w, T> {
    fn drop(&mut self) {
        // Explicitly cast mg into the InnerWriter that it guards in order for
        // split borrowing to work. Without this line the compiler thinks that
        // the usage of readers and blocking_readers are conflicting mutable borrows
        // https://doc.rust-lang.org/nomicon/borrow-splitting.html
        let writer: &mut Writer<_> = &mut self.writer;
        assert!(writer.blocking_readers.is_empty());

        // Swap the active and standby tables according to the Writer's
        // accounting.
        std::mem::swap(&mut writer.active_table, &mut writer.standby_table);

        for (key, table_and_epoch) in writer.readers.lock().iter_mut() {
            // Swap the active table for each Reader.
            let res = table_and_epoch.table.compare_exchange(
                writer.standby_table.as_mut() as *mut T,
                writer.active_table.as_mut() as *mut T,
                Ordering::SeqCst,
                Ordering::SeqCst,
            );
            assert_eq!(res, Ok(writer.standby_table.as_mut() as *mut T));

            // Make sure that swap occurs before recording the epoch.
            fence(Ordering::SeqCst);

            // Once the tables have been swapped, record the epoch of each
            // reader so that we will know if it is safe to update the new
            // standby table.
            let first_epoch_after_swap = table_and_epoch.epoch.load(Ordering::Acquire);
            if first_epoch_after_swap % 2 != 0 {
                // If the epoch is even, it means that there is no AsLockReadGuard
                // active.
                writer.blocking_readers.insert(key, first_epoch_after_swap);
            }
        }
    }
}

impl<'w, T> std::ops::Deref for AsLockWriteGuard<'w, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.writer.standby_table
    }
}

impl<'w, T> std::fmt::Debug for AsLockWriteGuard<'w, T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AsLockWriteGuard")
            .field("num_readers", &self.writer.readers.lock().len())
            .field("ops_to_replay", &self.writer.ops_to_replay.len())
            .field("standby_table", &self.writer.standby_table)
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
        T: Clone + std::fmt::Debug,
    {
        fn apply_first(&mut self, table: &'a mut Vec<T>) {
            dbg!(&table);
            table.push(self.value.clone());
            dbg!(&table);
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
    #[should_panic(expected = "Reader is not reentrant")]
    fn reader_not_reentrant() {
        let table = AsLockHandle::<Vec<i32>>::default();
        let _rg1 = table.read();
        let _rg2 = table.read();
    }

    #[test]
    fn writer_not_reentrant() {
        let table = AsLockHandle::<Vec<i32>>::from_identical(vec![], vec![]);
        let _wg = table.write();

        // If we uncomment this line the test fails due to Mutex not being
        // re-entrant. While it is well defined that the program will not
        // proceed it is not defined how exactly the failure will occur, so we
        // cannot expect a panic as this may deadlock and hang.
        //
        // let wg2 = table.write();
    }

    #[test]
    fn publish_update() {
        let table = AsLockHandle::<Vec<i32>>::new(vec![]);
        assert_eq!(table.read().len(), 0);

        {
            let mut wg = table.write();
            wg.update_tables(PushVec { value: 2 });
            assert_eq!(wg.len(), 1);
            assert_eq!(table.read().len(), 0);
        }

        // When the write guard is dropped it publishes the changes to the readers.
        assert_eq!(*table.read(), vec![2]);
    }

    #[test]
    fn update_tables_closure() {
        let table = AsLockHandle::<Vec<i32>>::default();
        assert_eq!(table.read().len(), 0);

        {
            let mut wg = table.write();
            wg.update_tables_closure(|vec| vec.push(2));
            assert_eq!(wg.len(), 1);
            assert_eq!(table.read().len(), 0);
        }

        // When the write guard is dropped it publishes the changes to the readers.
        assert_eq!(*table.read(), vec![2]);
    }

    #[test]
    fn multi_apply() {
        let table = AsLockHandle::<Vec<i32>>::default();
        {
            let mut wg = table.write();
            wg.update_tables(PushVec { value: 2 });
            wg.update_tables(PushVec { value: 3 });
            wg.update_tables(PushVec { value: 4 });
            wg.update_tables(PopVec {});
            wg.update_tables(PushVec { value: 5 });
        }
        assert_eq!(*table.read(), vec![2, 3, 5]);
    }

    #[test]
    fn multi_publish() {
        let table = AsLockHandle::<Vec<Box<i32>>>::default();
        {
            let mut wg = table.write();
            wg.update_tables(PushVec { value: Box::new(2) });
            wg.update_tables(PushVec { value: Box::new(3) });
            wg.update_tables(PopVec {});
            wg.update_tables(PushVec { value: Box::new(5) });
        }
        assert_eq!(*table.read(), vec![Box::new(2), Box::new(5)]);

        {
            let mut wg = table.write();
            wg.update_tables(PushVec { value: Box::new(9) });
            wg.update_tables(PushVec { value: Box::new(8) });
            wg.update_tables(PopVec {});
            wg.update_tables(PushVec { value: Box::new(7) });
        }
        assert_eq!(
            *table.read(),
            vec![Box::new(2), Box::new(5), Box::new(9), Box::new(7)]
        );

        table.write().update_tables(PopVec {});
        assert_eq!(*table.read(), vec![Box::new(2), Box::new(5), Box::new(9)]);
    }

    #[test]
    fn multi_thread() {
        let table = AsLockHandle::<Vec<i32>>::default();
        let handler = {
            let table = table.clone();
            thread::spawn(move || {
                while *table.read() != vec![2, 3, 5] {
                    // Since commits oly happen when an AsLockWriteGuard is dropped no reader
                    // will see this state.
                    assert_ne!(*table.read(), vec![2, 3, 4]);
                }

                // Show multiple readers in multiple threads.
                let handler = {
                    let table = table;
                    thread::spawn(move || while *table.read() != vec![2, 3, 5] {})
                };
                assert!(handler.join().is_ok());
            })
        };

        {
            let mut wg = table.write();
            wg.update_tables(PushVec { value: 2 });
            wg.update_tables(PushVec { value: 3 });
            wg.update_tables(PushVec { value: 4 });
            wg.update_tables(PopVec {});
            wg.update_tables(PushVec { value: 5 });
        }

        assert!(handler.join().is_ok());
    }

    #[test]
    fn writer_dropped() {
        // Show that when the Writer is dropped, Readers remain valid.
        let table;
        {
            table = AsLockHandle::<Vec<i32>>::default();

            {
                let mut wg = table.write();
                wg.update_tables(PushVec { value: 2 });
                wg.update_tables(PushVec { value: 3 });
                wg.update_tables(PushVec { value: 4 });
                wg.update_tables(PopVec {});
                wg.update_tables(PushVec { value: 5 });
            }
        }
        assert_eq!(*table.read(), vec![2, 3, 5]);
    }

    #[test]
    fn mutable_ref() {
        let table = AsLockHandle::<Vec<i32>>::default();

        {
            // Show that without giving a mutable interface we can still mutate
            // the underlying values in the table which will cause them to lose
            // consistency.
            let mut wg = table.write();
            wg.update_tables(PushVec { value: 2 });
            let mr = wg.update_tables(MutableRef {});
            *mr = 10;
        }

        assert_eq!(*table.read(), vec![10]);

        // This is bad and something clients must avoid. See comment on
        // UpdateTables trait for why this cannot be enforced by the library.
        assert_ne!(*table.read(), *table.write());
    }

    #[test]
    fn debug_str() {
        let table = AsLockHandle::<Vec<i32>>::default();
        assert_eq!(
            format!("{:?}", table),
            "AsLockHandle { num_readers: 1, num_ops_to_replay: 0, standby_table: [], active_table: [] }"
        );

        {
            let mut wg = table.write();
            wg.update_tables(PushVec { value: 2 });
            assert_eq!(
                format!("{:?}", wg),
                "AsLockWriteGuard { num_readers: 1, ops_to_replay: 1, standby_table: [2] }"
            );
        }

        // No second AsLockWriteGuard has been created, so we have yet to replay the
        // ops on the standby_table.
        assert_eq!(
            format!("{:?}", table),
            "AsLockHandle { num_readers: 1, num_ops_to_replay: 1, standby_table: [], active_table: [2] }"
        );
        assert_eq!(format!("{:?}", table.read()), "[2]");
    }
}
