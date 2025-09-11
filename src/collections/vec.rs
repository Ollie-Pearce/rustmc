use crate::UpdateTables;
// use std::collections::TryReserveError;
use std::ops::RangeBounds;

// Define the functions that the active_standby vector will have. Note that we
// only do this once, since both lockless & sync use the same UpdateTables
// trait.

struct Push<T> {
    value: T,
}

impl<'a, T> UpdateTables<'a, Vec<T>, ()> for Push<T>
where
    T: Clone,
{
    fn apply_first(&mut self, table: &'a mut Vec<T>) {
        table.push(self.value.clone())
    }
    fn apply_second(self, table: &mut Vec<T>) {
        table.push(self.value); // Move the value instead of cloning.
    }
}

struct Insert<T> {
    index: usize,
    element: T,
}

impl<'a, T> UpdateTables<'a, Vec<T>, ()> for Insert<T>
where
    T: Clone,
{
    fn apply_first(&mut self, table: &'a mut Vec<T>) {
        table.insert(self.index, self.element.clone())
    }
    fn apply_second(self, table: &mut Vec<T>) {
        // Move the value instead of cloning.
        table.insert(self.index, self.element)
    }
}

struct Append<T> {
    value: Vec<T>,
}

impl<'a, T> UpdateTables<'a, Vec<T>, ()> for Append<T>
where
    T: Clone,
{
    fn apply_first(&mut self, table: &'a mut Vec<T>) {
        table.append(&mut self.value.clone())
    }
    fn apply_second(mut self, table: &mut Vec<T>) {
        table.append(&mut self.value);
    }
}

struct ResizeWith<F> {
    new_len: usize,
    f: F,
}

impl<'a, T, F> UpdateTables<'a, Vec<T>, ()> for ResizeWith<F>
where
    F: Clone + FnMut() -> T,
{
    fn apply_first(&mut self, table: &'a mut Vec<T>) {
        table.resize_with(self.new_len, self.f.clone())
    }
    fn apply_second(self, table: &mut Vec<T>) {
        table.resize_with(self.new_len, self.f);
    }
}

struct ExtendFromWithin<R> {
    range: R,
}

impl<'a, T, R> UpdateTables<'a, Vec<T>, ()> for ExtendFromWithin<R>
where
    R: 'static + Clone + RangeBounds<usize>,
    T: Clone,
{
    fn apply_first(&mut self, table: &'a mut Vec<T>) {
        table.extend_from_within(self.range.clone())
    }
    fn apply_second(self, table: &mut Vec<T>) {
        table.extend_from_within(self.range);
    }
}

struct Drain<R> {
    range: R,
}

impl<'a, T, R> UpdateTables<'a, Vec<T>, std::vec::Drain<'a, T>> for Drain<R>
where
    R: 'static + Clone + RangeBounds<usize>,
{
    fn apply_first(&mut self, table: &'a mut Vec<T>) -> std::vec::Drain<'a, T> {
        table.drain(self.range.clone())
    }
    fn apply_second(self, table: &mut Vec<T>) {
        table.drain(self.range);
    }
}

struct Retain<F> {
    f: F,
}

impl<'a, T, F> UpdateTables<'a, Vec<T>, ()> for Retain<F>
where
    F: 'static + Clone + FnMut(&T) -> bool,
{
    fn apply_first(&mut self, table: &'a mut Vec<T>) {
        table.retain(self.f.clone())
    }
    fn apply_second(self, table: &mut Vec<T>) {
        table.retain(self.f)
    }
}

struct DedupByKey<F> {
    f: F,
}

impl<'a, T, F, K> UpdateTables<'a, Vec<T>, ()> for DedupByKey<F>
where
    F: 'static + Clone + FnMut(&mut T) -> K,
    K: PartialEq<K>,
{
    fn apply_first(&mut self, table: &'a mut Vec<T>) {
        table.dedup_by_key(self.f.clone())
    }
    fn apply_second(self, table: &mut Vec<T>) {
        table.dedup_by_key(self.f)
    }
}

struct DedupBy<F> {
    f: F,
}

impl<'a, T, F> UpdateTables<'a, Vec<T>, ()> for DedupBy<F>
where
    F: 'static + Clone + FnMut(&mut T, &mut T) -> bool,
{
    fn apply_first(&mut self, table: &'a mut Vec<T>) {
        table.dedup_by(self.f.clone())
    }
    fn apply_second(self, table: &mut Vec<T>) {
        table.dedup_by(self.f)
    }
}

struct SortBy<F> {
    f: F,
}

impl<'a, T, F> UpdateTables<'a, Vec<T>, ()> for SortBy<F>
where
    F: Clone + FnMut(&T, &T) -> std::cmp::Ordering,
{
    fn apply_first(&mut self, table: &'a mut Vec<T>) {
        table.sort_by(self.f.clone())
    }
    fn apply_second(self, table: &mut Vec<T>) {
        table.sort_by(self.f)
    }
}

/// Implementation of Vec for use in the active_standby model.
/// `lockless::AsLockHandle<T>`, should function similarly to
/// `Arc<RwLock<Vec<T>>>`.
pub mod lockless {
    use super::*;
    crate::generate_lockless_aslockhandle!(Vec<T>);

    impl<'w, T> AsLockWriteGuard<'w, T>
    where
        T: 'static + Clone + Send,
    {
        pub fn push(&mut self, value: T) {
            self.guard.update_tables(Push { value })
        }

        pub fn append(&mut self, other: &mut Vec<T>) {
            self.guard.update_tables(Append {
                value: other.drain(..).collect(),
            })
        }

        pub fn insert(&mut self, index: usize, element: T) {
            self.guard.update_tables(Insert { index, element })
        }
    }

    impl<'w, T> AsLockWriteGuard<'w, T> {
        pub fn clear(&mut self) {
            self.guard.update_tables_closure(move |table| table.clear())
        }

        pub fn pop(&mut self) -> Option<T> {
            self.guard.update_tables_closure(move |table| table.pop())
        }

        pub fn reverse(&mut self) {
            self.guard
                .update_tables_closure(move |table| table.reverse())
        }

        pub fn reserve(&mut self, additional: usize) {
            self.guard
                .update_tables_closure(move |table| table.reserve(additional))
        }

        pub fn reserve_exact(&mut self, additional: usize) {
            self.guard
                .update_tables_closure(move |table| table.reserve_exact(additional))
        }

        // pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        //     self.guard
        //         .update_tables_closure(move |table| table.try_reserve(additional))
        // }

        // pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        //     self.guard
        //         .update_tables_closure(move |table| table.try_reserve_exact(additional))
        // }

        pub fn shrink_to_fit(&mut self) {
            self.guard
                .update_tables_closure(move |table| table.shrink_to_fit())
        }

        pub fn shrink_to(&mut self, min_capacity: usize) {
            self.guard
                .update_tables_closure(move |table| table.shrink_to(min_capacity))
        }

        pub fn truncate(&mut self, len: usize) {
            self.guard
                .update_tables_closure(move |table| table.truncate(len))
        }

        pub fn swap_remove(&mut self, index: usize) -> T {
            self.guard
                .update_tables_closure(move |table| table.swap_remove(index))
        }

        pub fn remove(&mut self, index: usize) -> T {
            self.guard
                .update_tables_closure(move |table| table.remove(index))
        }

        pub fn extend_from_within<R>(&mut self, range: R)
        where
            R: 'static + Clone + Send + RangeBounds<usize>,
            T: Clone,
        {
            self.guard.update_tables(ExtendFromWithin { range })
        }

        pub fn retain<F>(&mut self, f: F)
        where
            F: 'static + Clone + Send + FnMut(&T) -> bool,
        {
            self.guard.update_tables(Retain { f })
        }

        pub fn resize_with<F>(&mut self, new_len: usize, f: F)
        where
            F: 'static + Clone + Send + FnMut() -> T,
        {
            self.guard.update_tables(ResizeWith { new_len, f })
        }

        pub fn dedup_by_key<F, K>(&mut self, f: F)
        where
            F: 'static + Clone + Send + FnMut(&mut T) -> K,
            K: 'static + PartialEq<K>, // Shouldn't need a lifetime.
        {
            self.guard.update_tables(DedupByKey { f })
        }

        pub fn dedup_by<F>(&mut self, f: F)
        where
            F: 'static + Clone + Send + FnMut(&mut T, &mut T) -> bool,
        {
            self.guard.update_tables(DedupBy { f })
        }

        pub fn sort_by<F>(&mut self, f: F)
        where
            F: 'static + Clone + Send + FnMut(&T, &T) -> std::cmp::Ordering,
        {
            self.guard.update_tables(SortBy { f })
        }
    }

    impl<'w, T> AsLockWriteGuard<'w, T>
    where
        T: Ord,
    {
        pub fn sort(&mut self) {
            self.guard.update_tables_closure(move |table| table.sort())
        }

        pub fn sort_unstable(&mut self) {
            self.guard
                .update_tables_closure(move |table| table.sort_unstable())
        }
    }

    impl<'w, T> AsLockWriteGuard<'w, T>
    where
        T: PartialEq<T>,
    {
        pub fn dedup(&mut self) {
            self.guard.update_tables_closure(|table| table.dedup())
        }
    }

    impl<'w, 'a, T> AsLockWriteGuard<'w, T> {
        pub fn drain<R>(&'a mut self, range: R) -> std::vec::Drain<'a, T>
        where
            R: 'static + Clone + Send + RangeBounds<usize>,
        {
            self.guard.update_tables(Drain { range })
        }
    }
}

/// Implementation of Vec for use in the active_standby model.
/// `sync::AsLock<T>`, should function similarly to `RwLock<Vec<T>>`.
pub mod sync {
    use super::*;
    crate::generate_sync_aslock!(Vec<T>);

    impl<'w, T> AsLockWriteGuard<'w, T>
    where
        T: 'static + Clone + Send,
    {
        pub fn push(&mut self, value: T) {
            self.guard.update_tables(Push { value })
        }

        pub fn append(&mut self, other: &mut Vec<T>) {
            self.guard.update_tables(Append {
                value: other.drain(..).collect(),
            })
        }

        pub fn insert(&mut self, index: usize, element: T) {
            self.guard.update_tables(Insert { index, element })
        }
    }

    impl<'w, T> AsLockWriteGuard<'w, T> {
        pub fn clear(&mut self) {
            self.guard.update_tables_closure(move |table| table.clear())
        }
        pub fn pop(&mut self) -> Option<T> {
            self.guard.update_tables_closure(move |table| table.pop())
        }

        pub fn reserve(&mut self, additional: usize) {
            self.guard
                .update_tables_closure(move |table| table.reserve(additional))
        }

        pub fn reserve_exact(&mut self, additional: usize) {
            self.guard
                .update_tables_closure(move |table| table.reserve_exact(additional))
        }

        // pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        //     self.guard
        //         .update_tables_closure(move |table| table.try_reserve(additional))
        // }

        // pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        //     self.guard
        //         .update_tables_closure(move |table| table.try_reserve_exact(additional))
        // }

        pub fn shrink_to_fit(&mut self) {
            self.guard
                .update_tables_closure(move |table| table.shrink_to_fit())
        }

        pub fn shrink_to(&mut self, min_capacity: usize) {
            self.guard
                .update_tables_closure(move |table| table.shrink_to(min_capacity))
        }

        pub fn truncate(&mut self, len: usize) {
            self.guard
                .update_tables_closure(move |table| table.truncate(len))
        }

        pub fn swap_remove(&mut self, index: usize) -> T {
            self.guard
                .update_tables_closure(move |table| table.swap_remove(index))
        }

        pub fn remove(&mut self, index: usize) -> T {
            self.guard
                .update_tables_closure(move |table| table.remove(index))
        }

        pub fn extend_from_within<R>(&mut self, range: R)
        where
            R: 'static + Clone + Send + RangeBounds<usize>,
            T: Clone,
        {
            self.guard.update_tables(ExtendFromWithin { range })
        }

        pub fn retain<F>(&mut self, f: F)
        where
            F: 'static + Clone + Send + FnMut(&T) -> bool,
        {
            self.guard.update_tables(Retain { f })
        }

        pub fn resize_with<F>(&mut self, new_len: usize, f: F)
        where
            F: 'static + Clone + Send + FnMut() -> T,
        {
            self.guard.update_tables(ResizeWith { new_len, f })
        }

        pub fn dedup_by_key<F, K>(&mut self, f: F)
        where
            F: 'static + Clone + Send + FnMut(&mut T) -> K,
            K: 'static + PartialEq<K>, // Shouldn't need a lifetime.
        {
            self.guard.update_tables(DedupByKey { f })
        }

        pub fn dedup_by<F>(&mut self, f: F)
        where
            F: 'static + Clone + Send + FnMut(&mut T, &mut T) -> bool,
        {
            self.guard.update_tables(DedupBy { f })
        }

        pub fn sort_by<F>(&mut self, f: F)
        where
            F: 'static + Clone + Send + FnMut(&T, &T) -> std::cmp::Ordering,
        {
            self.guard.update_tables(SortBy { f })
        }
    }

    impl<'w, T> AsLockWriteGuard<'w, T>
    where
        T: Ord,
    {
        pub fn sort(&mut self) {
            self.guard.update_tables_closure(move |table| table.sort())
        }

        pub fn sort_unstable(&mut self) {
            self.guard
                .update_tables_closure(move |table| table.sort_unstable())
        }
    }

    impl<'w, T> AsLockWriteGuard<'w, T>
    where
        T: PartialEq<T>,
    {
        pub fn dedup(&mut self) {
            self.guard.update_tables_closure(|table| table.dedup())
        }
    }

    impl<'w, 'a, T> AsLockWriteGuard<'w, T> {
        pub fn drain<R>(&'a mut self, range: R) -> std::vec::Drain<'a, T>
        where
            R: 'static + Clone + Send + RangeBounds<usize>,
        {
            self.guard.update_tables(Drain { range })
        }
    }
}

#[cfg(test)]
mod lockless_test {
    use super::*;
    use crate::assert_tables_eq;

    #[test]
    fn push() {
        let lock1 = lockless::AsLockHandle::<i32>::default();
        let lock2 = lock1.clone();
        assert_eq!(lock1.read().len(), 0);

        {
            let mut wg = lock1.write();
            wg.push(2);
            assert_eq!(wg.len(), 1);
            assert_eq!(lock2.read().len(), 0);
        }

        // When the write guard is dropped it publishes the changes to the readers.
        assert_tables_eq!(lock1, vec![2]);
    }

    #[test]
    fn clear() {
        let aslock = lockless::AsLockHandle::<i32>::default();
        assert_eq!(aslock.read().len(), 0);

        {
            let aslock2 = aslock.clone();
            let mut wg = aslock.write();
            wg.push(2);
            assert_eq!(wg.len(), 1);
            assert_eq!(aslock2.read().len(), 0);
        }

        // When the write guard is dropped it publishes the changes to the readers.
        assert_tables_eq!(aslock, vec![2]);

        aslock.write().clear();
        assert_tables_eq!(aslock, vec![]);
    }

    #[test]
    fn pop() {
        let table = lockless::AsLockHandle::<i32>::default();
        {
            let mut wg = table.write();
            wg.push(2);
            wg.push(3);
            wg.pop();
            wg.push(4);
        }

        // When the write guard is dropped it publishes the changes to the readers.
        assert_tables_eq!(table, vec![2, 4]);
    }

    #[test]
    fn append() {
        let table = lockless::AsLockHandle::<i32>::default();
        let mut other = vec![1, 2, 3];
        table.write().append(&mut other);
        assert!(other.is_empty());
        assert_tables_eq!(table, vec![1, 2, 3]);
    }

    #[test]
    fn indirect_type() {
        let table = lockless::AsLockHandle::<Box<i32>>::default();
        table.write().push(Box::new(2));
        assert_tables_eq!(table, vec![Box::new(2)]);
    }

    #[test]
    fn reverse() {
        let table = lockless::AsLockHandle::new(vec![1, 2, 3]);
        table.write().reverse();
        assert_tables_eq!(table, vec![3, 2, 1]);
    }

    #[test]
    fn reserve() {
        let table = lockless::AsLockHandle::<i32>::default();
        table.write().reserve(123);

        assert!(table.read().capacity() >= 123);
        assert!(table.write().capacity() >= 123);
        assert!(table.read().capacity() >= 123);
    }

    #[test]
    fn reserve_exact() {
        let table = lockless::AsLockHandle::<i32>::default();
        table.write().reserve_exact(123);

        assert_eq!(table.read().capacity(), 123);
        assert_eq!(table.write().capacity(), 123);
        assert_eq!(table.read().capacity(), 123);
    }

    // #[test]
    // fn try_reserve() {
    //     let table = lockless::AsLockHandle::<i32>::default();
    //     assert!(table.write().try_reserve(123).is_ok());

    //     assert!(table.read().capacity() >= 123);
    //     assert!(table.write().capacity() >= 123);
    //     assert!(table.read().capacity() >= 123);
    // }

    // #[test]
    // fn try_reserve_exact() {
    //     let table = lockless::AsLockHandle::<i32>::default();
    //     assert!(table.write().try_reserve_exact(123).is_ok());

    //     assert_eq!(table.read().capacity(), 123);
    //     assert_eq!(table.write().capacity(), 123);
    //     assert_eq!(table.read().capacity(), 123);
    // }

    #[test]
    fn shrink_to_fit() {
        let table = lockless::AsLockHandle::<i32>::default();

        {
            let mut wg = table.write();
            wg.reserve_exact(123);
            wg.push(2);
            wg.push(3);
            wg.shrink_to_fit();
        }

        assert_eq!(table.read().capacity(), 2);
        assert_eq!(table.write().capacity(), 2);
        assert_eq!(table.read().capacity(), 2);
    }

    #[test]
    fn shrink_to() {
        let table = lockless::AsLockHandle::<i32>::default();

        {
            let mut wg = table.write();
            wg.reserve_exact(123);
            wg.push(2);
            wg.push(3);
            wg.shrink_to(10);
        }

        assert_eq!(table.read().capacity(), 10);
        assert_eq!(table.write().capacity(), 10);
        assert_eq!(table.read().capacity(), 10);
    }

    #[test]
    fn truncate() {
        let table = lockless::AsLockHandle::<i32>::default();

        {
            let mut wg = table.write();
            for i in 0..10 {
                wg.push(i);
            }
            wg.truncate(3);
        }

        assert_tables_eq!(table, vec![0, 1, 2]);
    }

    #[test]
    fn swap_remove() {
        let table = lockless::AsLockHandle::<i32>::default();

        {
            let mut wg = table.write();
            for i in 0..5 {
                wg.push(i);
            }
            assert_eq!(wg.swap_remove(2), 2);
        }

        assert_tables_eq!(table, vec![0, 1, 4, 3]);
    }

    #[test]
    fn remove() {
        let table = lockless::AsLockHandle::<i32>::default();

        {
            let mut wg = table.write();
            for i in 0..5 {
                wg.push(i);
            }
            assert_eq!(wg.remove(2), 2);
        }

        assert_tables_eq!(table, vec![0, 1, 3, 4]);
    }

    #[test]
    fn extend_from_within() {
        let table = lockless::AsLockHandle::<i32>::new(vec![1, 2, 3]);
        table.write().extend_from_within(1..);
        assert_tables_eq!(table, vec![1, 2, 3, 2, 3]);
    }

    #[test]
    fn insert() {
        let table = lockless::AsLockHandle::<i32>::default();
        let table2 = table.clone();

        {
            let mut wg = table.write();
            for i in 0..5 {
                wg.push(i);
            }
            wg.insert(2, 10);
            assert_eq!(*wg, vec![0, 1, 10, 2, 3, 4]);
            assert_eq!(*table2.read(), vec![]);
        }

        assert_tables_eq!(table, vec![0, 1, 10, 2, 3, 4]);
    }

    #[test]
    fn retain() {
        let table = lockless::AsLockHandle::<i32>::default();

        {
            let mut wg = table.write();
            for i in 0..5 {
                wg.push(i);
            }
            wg.retain(|element| element % 2 == 0);
        }

        assert_tables_eq!(table, vec![0, 2, 4]);
    }

    #[test]
    fn resize_with() {
        let table = lockless::AsLockHandle::<i32>::new(vec![1, 2]);
        let mut i = 2;
        table.write().resize_with(4, move || {
            i += 1;
            i
        });
        assert_tables_eq!(table, vec![1, 2, 3, 4]);
    }

    #[test]
    fn sort_by() {
        let table = lockless::AsLockHandle::new(vec![-5, 4, 1, -3, 2]);
        table.write().sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_tables_eq!(table, vec![-5, -3, 1, 2, 4]);
    }

    #[test]
    fn sort() {
        let table = lockless::AsLockHandle::new(vec![-5, 4, 1, -3, 2]);
        table.write().sort();
        assert_tables_eq!(table, vec![-5, -3, 1, 2, 4]);
    }

    #[test]
    fn sort_unstable() {
        let table = lockless::AsLockHandle::new(vec![-5, 4, 1, -3, 2]);
        table.write().sort_unstable();
        assert_tables_eq!(table, vec![-5, -3, 1, 2, 4]);
    }

    #[test]
    fn dedup() {
        let table = lockless::AsLockHandle::<i32>::new(vec![1, 1, 2, 3, 3, 2]);
        table.write().dedup();
        assert_tables_eq!(table, vec![1, 2, 3, 2]);
    }

    #[test]
    fn dedup_by_key() {
        let table = lockless::AsLockHandle::<i32>::new(vec![1, 1, 2, 3, 3, 2]);
        table.write().dedup_by_key(|a| *a);
        assert_tables_eq!(table, vec![1, 2, 3, 2]);
    }

    #[test]
    fn dedup_by() {
        let table = lockless::AsLockHandle::<i32>::new(vec![1, 1, 2, 3, 3, 2]);
        table.write().dedup_by(|a, b| a == b);
        assert_tables_eq!(table, vec![1, 2, 3, 2]);
    }

    #[test]
    fn drain() {
        let table = lockless::AsLockHandle::<i32>::new(vec![]);

        {
            let mut wg = table.write();
            for i in 0..5 {
                wg.push(i + 1);
            }
            assert_eq!(wg.drain(1..4).collect::<Vec<_>>(), vec![2, 3, 4]);
        }

        assert_tables_eq!(table, vec![1, 5]);
    }

    #[test]
    fn lifetimes() {
        let table = lockless::AsLockHandle::<i32>::from_identical(vec![], vec![]);

        {
            let mut wg = table.write();
            for i in 0..5 {
                wg.push(i + 1);
            }
            // Switching the order of 'drain' and 'swapped' fails to compile due
            // to the borrow checker, since 'drain' is tied to the lifetime of
            // table.
            let swapped = wg.swap_remove(1);
            let drain = wg.drain(1..4);
            assert_eq!(drain.collect::<Vec<_>>(), vec![5, 3, 4]);
            assert_eq!(swapped, 2);
        }

        assert_tables_eq!(table, vec![1]);
    }

    #[test]
    fn debug_str() {
        let table = super::lockless::AsLockHandle::<i32>::default();
        {
            table.write().push(12);
        }

        assert_eq!(format!("{:?}", table), "AsLockHandle { num_readers: 1, num_ops_to_replay: 1, standby_table: [], active_table: [12] }",);
        assert_eq!(
            format!("{:?}", table.write()),
            "AsLockWriteGuard { num_readers: 1, ops_to_replay: 0, standby_table: [12] }",
        );
        assert_eq!(format!("{:?}", table.read()), "[12]",);
    }

    #[test]
    fn update_tables_raw() {
        let table = super::lockless::AsLockHandle::<i32>::default();
        {
            table.write().update_tables(Push { value: 1 });
            table.write().update_tables(Push { value: 2 });
            table.write().update_tables_closure(|v| {
                for x in v.iter_mut() {
                    *x += 1;
                }
            });
        }
        assert_eq!(*table.read(), vec![2, 3]);
    }
}

#[cfg(test)]
mod sync_test {
    use super::*;
    use crate::assert_tables_eq;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn push() {
        let lock1 = Arc::new(sync::AsLock::<i32>::default());
        let lock2 = Arc::clone(&lock1);
        assert_eq!(lock1.read().len(), 0);

        {
            let mut wg = lock1.write();
            wg.push(2);
            assert_eq!(wg.len(), 1);
            {
                // Perform check in another thread to avoid potential deadlock
                // (calling both read and write on aslock at the same time).
                assert!(thread::spawn(move || {
                    assert_eq!(lock2.read().len(), 0);
                })
                .join()
                .is_ok());
            }
        }

        // When the write guard is dropped it publishes the changes to the readers.
        assert_tables_eq!(lock1, vec![2]);
    }

    #[test]
    fn clear() {
        let table = Arc::new(sync::AsLock::<i32>::default());
        assert_eq!(table.read().len(), 0);

        {
            let mut wg = table.write();
            wg.push(2);
            assert_eq!(wg.len(), 1);
            {
                // Perform check in another thread to avoid potential deadlock
                // (calling both read and write on aslock at the same time).
                let table = Arc::clone(&table);
                assert!(thread::spawn(move || {
                    assert_eq!(table.read().len(), 0);
                })
                .join()
                .is_ok());
            }
        }

        // When the write guard is dropped it publishes the changes to the readers.
        assert_tables_eq!(table, vec![2]);

        table.write().clear();
        assert_tables_eq!(table, vec![]);
    }

    #[test]
    fn pop() {
        let table = Arc::new(sync::AsLock::<i32>::default());
        {
            let mut wg = table.write();
            wg.push(2);
            wg.push(3);
            wg.pop();
            wg.push(4);
        }

        // When the write guard is dropped it publishes the changes to the readers.
        assert_tables_eq!(table, vec![2, 4]);
    }

    #[test]
    fn append() {
        let table = sync::AsLock::<i32>::default();
        let mut other = vec![1, 2, 3];
        table.write().append(&mut other);
        assert!(other.is_empty());
        assert_tables_eq!(table, vec![1, 2, 3]);
    }

    #[test]
    fn indirect_type() {
        let table = sync::AsLock::<Box<i32>>::default();
        table.write().push(Box::new(2));
        assert_tables_eq!(table, vec![Box::new(2)]);
    }

    #[test]
    fn reverse() {
        let table = lockless::AsLockHandle::new(vec![1, 2, 3]);
        table.write().reverse();
        assert_tables_eq!(table, vec![3, 2, 1]);
    }

    #[test]
    fn reserve() {
        let table = Arc::new(sync::AsLock::<i32>::default());
        table.write().reserve(123);

        assert!(table.read().capacity() >= 123);
        assert!(table.write().capacity() >= 123);
        assert!(table.read().capacity() >= 123);
    }

    #[test]
    fn reserve_exact() {
        let table = Arc::new(sync::AsLock::<i32>::default());
        table.write().reserve_exact(123);

        assert_eq!(table.read().capacity(), 123);
        assert_eq!(table.write().capacity(), 123);
        assert_eq!(table.read().capacity(), 123);
    }

    // #[test]
    // fn try_reserve() {
    //     let table = Arc::new(sync::AsLock::<i32>::default());
    //     assert!(table.write().try_reserve(123).is_ok());

    //     assert!(table.read().capacity() >= 123);
    //     assert!(table.write().capacity() >= 123);
    //     assert!(table.read().capacity() >= 123);
    // }

    // #[test]
    // fn try_reserve_exact() {
    //     let table = Arc::new(sync::AsLock::<i32>::default());
    //     assert!(table.write().try_reserve_exact(123).is_ok());

    //     assert_eq!(table.read().capacity(), 123);
    //     assert_eq!(table.write().capacity(), 123);
    //     assert_eq!(table.read().capacity(), 123);
    // }

    #[test]
    fn shrink_to_fit() {
        let table = Arc::new(sync::AsLock::<i32>::default());

        {
            let mut wg = table.write();
            wg.reserve_exact(123);
            wg.push(2);
            wg.push(3);
            wg.shrink_to_fit();
        }

        assert_eq!(table.read().capacity(), 2);
        assert_eq!(table.write().capacity(), 2);
        assert_eq!(table.read().capacity(), 2);
    }

    #[test]
    fn shrink_to() {
        let table = sync::AsLock::<i32>::default();

        {
            let mut wg = table.write();
            wg.reserve_exact(123);
            wg.push(2);
            wg.push(3);
            wg.shrink_to(10);
        }

        assert_eq!(table.read().capacity(), 10);
        assert_eq!(table.write().capacity(), 10);
        assert_eq!(table.read().capacity(), 10);
    }

    #[test]
    fn truncate() {
        let table = sync::AsLock::<i32>::new(vec![0, 1, 2, 3, 4]);
        table.write().truncate(3);
        assert_tables_eq!(table, vec![0, 1, 2]);
    }

    #[test]
    fn swap_remove() {
        let table = sync::AsLock::<i32>::new(vec![0, 1, 2, 3, 4]);
        assert_eq!(table.write().swap_remove(2), 2);
        assert_tables_eq!(table, vec![0, 1, 4, 3]);
    }

    #[test]
    fn remove() {
        let table = sync::AsLock::<i32>::new(vec![0, 1, 2, 3, 4]);
        assert_eq!(table.write().remove(2), 2);
        assert_tables_eq!(table, vec![0, 1, 3, 4]);
    }

    #[test]
    fn extend_from_within() {
        let table = sync::AsLock::<i32>::new(vec![1, 2, 3]);
        table.write().extend_from_within(1..);
        assert_tables_eq!(table, vec![1, 2, 3, 2, 3]);
    }

    #[test]
    fn insert() {
        let table = Arc::new(sync::AsLock::<i32>::default());

        {
            let mut wg = table.write();
            for i in 0..5 {
                wg.push(i);
            }
            wg.insert(2, 10);
            assert_eq!(*wg, vec![0, 1, 10, 2, 3, 4]);
            {
                // Perform check in another thread to avoid potential deadlock
                // (calling both read and write on aslock at the same time).
                let table = Arc::clone(&table);
                assert!(thread::spawn(move || {
                    assert_eq!(*table.read(), vec![]);
                })
                .join()
                .is_ok());
            }
        }

        assert_tables_eq!(table, vec![0, 1, 10, 2, 3, 4]);
    }

    #[test]
    fn retain() {
        let table = sync::AsLock::<i32>::new(vec![0, 1, 2, 3, 4]);
        table.write().retain(|element| element % 2 == 0);
        assert_tables_eq!(table, vec![0, 2, 4]);
    }

    #[test]
    fn resize_with() {
        let table = sync::AsLock::<i32>::new(vec![1, 2]);
        let mut i = 2;
        table.write().resize_with(4, move || {
            i += 1;
            i
        });
        assert_tables_eq!(table, vec![1, 2, 3, 4]);
    }

    #[test]
    fn sort_by() {
        let table = sync::AsLock::new(vec![-5, 4, 1, -3, 2]);
        table.write().sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_tables_eq!(table, vec![-5, -3, 1, 2, 4]);
    }

    #[test]
    fn sort() {
        let table = sync::AsLock::new(vec![-5, 4, 1, -3, 2]);
        table.write().sort();
        assert_tables_eq!(table, vec![-5, -3, 1, 2, 4]);
    }

    #[test]
    fn sort_unstable() {
        let table = sync::AsLock::new(vec![-5, 4, 1, -3, 2]);
        table.write().sort_unstable();
        assert_tables_eq!(table, vec![-5, -3, 1, 2, 4]);
    }

    #[test]
    fn dedup() {
        let table = sync::AsLock::<i32>::new(vec![1, 1, 2, 3, 3, 2]);
        table.write().dedup();
        assert_tables_eq!(table, vec![1, 2, 3, 2]);
    }

    #[test]
    fn dedup_by_key() {
        let table = sync::AsLock::<i32>::new(vec![1, 1, 2, 3, 3, 2]);
        table.write().dedup_by_key(|a| *a);
        assert_tables_eq!(table, vec![1, 2, 3, 2]);
    }

    #[test]
    fn dedup_by() {
        let table = sync::AsLock::<i32>::new(vec![1, 1, 2, 3, 3, 2]);
        table.write().dedup_by(|a, b| a == b);
        assert_tables_eq!(table, vec![1, 2, 3, 2]);
    }

    #[test]
    fn drain() {
        let table = sync::AsLock::<i32>::new(vec![0, 1, 2, 3, 4]);
        assert_eq!(table.write().drain(1..4).collect::<Vec<_>>(), vec![1, 2, 3]);
        assert_tables_eq!(table, vec![0, 4]);
    }

    #[test]
    fn lifetimes() {
        let table = sync::AsLock::<i32>::from_identical(vec![], vec![]);

        {
            let mut wg = table.write();
            for i in 0..5 {
                wg.push(i + 1);
            }
            // Switching the order of 'drain' and 'swapped' fails to compile due
            // to the borrow checker, since 'drain' is tied to the lifetime of
            // table.
            let swapped = wg.swap_remove(1);
            let drain = wg.drain(1..4);
            assert_eq!(drain.collect::<Vec<_>>(), vec![5, 3, 4]);
            assert_eq!(swapped, 2);
        }

        assert_tables_eq!(table, vec![1]);
    }

    #[test]
    fn debug_str() {
        let table = Arc::new(sync::AsLock::<i32>::default());
        table.write().push(12);

        assert_eq!(
            format!("{:?}", table),
            "AsLock { num_ops_to_replay: 1, standby_table: [12], active_table: [12] }",
        );
        assert_eq!(
            format!("{:?}", table.write()),
            "AsLockWriteGuard { num_ops_to_replay: 0, standby_table: [12] }",
        );
        assert_eq!(format!("{:?}", table.read()), "[12]",);
    }

    #[test]
    fn update_tables_raw() {
        let table = Arc::new(sync::AsLock::<i32>::default());
        {
            table.write().update_tables(Push { value: 1 });
            table.write().update_tables(Push { value: 2 });
            table.write().update_tables_closure(|v| {
                for x in v.iter_mut() {
                    *x += 1;
                }
            });
        }
        assert_eq!(*table.read(), vec![2, 3]);
    }
}
