use crate::UpdateTables;
use std::collections::BTreeSet;

struct Insert<T> {
    value: T,
}

impl<'a, T> UpdateTables<'a, BTreeSet<T>, bool> for Insert<T>
where
    T: Ord + Clone,
{
    fn apply_first(&mut self, table: &'a mut BTreeSet<T>) -> bool {
        table.insert(self.value.clone())
    }
    fn apply_second(self, table: &mut BTreeSet<T>) {
        // Move the value instead of cloning.
        table.insert(self.value);
    }
}

struct Replace<T> {
    value: T,
}

impl<'a, T> UpdateTables<'a, BTreeSet<T>, Option<T>> for Replace<T>
where
    T: Ord + Clone,
{
    fn apply_first(&mut self, table: &'a mut BTreeSet<T>) -> Option<T> {
        table.replace(self.value.clone())
    }
    fn apply_second(self, table: &mut BTreeSet<T>) {
        // Move the value instead of cloning.
        table.replace(self.value);
    }
}

struct Append<T> {
    other: BTreeSet<T>,
}

impl<'a, T> UpdateTables<'a, BTreeSet<T>, ()> for Append<T>
where
    T: Ord + Clone,
{
    fn apply_first(&mut self, table: &'a mut BTreeSet<T>) {
        for k in self.other.iter() {
            table.insert(k.clone());
        }
    }
    fn apply_second(mut self, table: &mut BTreeSet<T>) {
        table.append(&mut self.other);
    }
}

struct Retain<F> {
    f: F,
}

impl<'a, T, F> UpdateTables<'a, BTreeSet<T>, ()> for Retain<F>
where
    T: Ord,
    F: Clone + FnMut(&T) -> bool,
{
    fn apply_first(&mut self, table: &'a mut BTreeSet<T>) {
        table.retain(self.f.clone())
    }
    fn apply_second(self, table: &mut BTreeSet<T>) {
        table.retain(self.f);
    }
}

/// Implementation of BTreeSet for use in the active_standby model.
/// `lockless::AsLockHandle<T>`, should function similarly to
/// `Arc<RwLock<BTreeSet<T>>>`.
pub mod lockless {
    use super::*;
    crate::generate_lockless_aslockhandle!(BTreeSet<T>);

    impl<'w, T> AsLockWriteGuard<'w, T>
    where
        T: 'static + Ord + Clone + Send,
    {
        pub fn insert(&mut self, value: T) -> bool {
            self.guard.update_tables(Insert { value })
        }

        pub fn replace(&mut self, value: T) -> Option<T> {
            self.guard.update_tables(Replace { value })
        }

        pub fn clear(&mut self) {
            self.guard.update_tables_closure(move |table| table.clear())
        }

        pub fn remove<Q>(&mut self, value_like: Q) -> bool
        where
            T: std::borrow::Borrow<Q>,
            Q: 'static + Ord + Send,
        {
            self.guard
                .update_tables_closure(move |table| table.remove(&value_like))
        }

        pub fn take<Q>(&mut self, value_like: Q) -> Option<T>
        where
            T: std::borrow::Borrow<Q>,
            Q: 'static + Ord + Send,
        {
            self.guard
                .update_tables_closure(move |table| table.take(&value_like))
        }

        pub fn append(&mut self, other: BTreeSet<T>) {
            self.guard.update_tables(Append { other })
        }

        pub fn retain<F>(&mut self, f: F)
        where
            F: 'static + Send + Clone + FnMut(&T) -> bool,
        {
            self.guard.update_tables(Retain { f })
        }
    }
}

/// Implementation of BTreeSet for use in the active_standby model.
/// `sync::AsLock<T>`, should function similarly to `RwLock<BTreeSet<T>>`.
pub mod sync {
    use super::*;
    crate::generate_sync_aslock!(BTreeSet<T>);

    impl<'w, T> AsLockWriteGuard<'w, T>
    where
        T: 'static + Ord + Clone + Send,
    {
        pub fn insert(&mut self, value: T) -> bool {
            self.guard.update_tables(Insert { value })
        }

        pub fn replace(&mut self, value: T) -> Option<T> {
            self.guard.update_tables(Replace { value })
        }

        pub fn clear(&mut self) {
            self.guard.update_tables_closure(move |table| table.clear())
        }

        pub fn remove<Q>(&mut self, value_like: Q) -> bool
        where
            T: std::borrow::Borrow<Q>,
            Q: 'static + Ord + Send,
        {
            self.guard
                .update_tables_closure(move |table| table.remove(&value_like))
        }

        pub fn take<Q>(&mut self, value_like: Q) -> Option<T>
        where
            T: std::borrow::Borrow<Q>,
            Q: 'static + Ord + Send,
        {
            self.guard
                .update_tables_closure(move |table| table.take(&value_like))
        }

        pub fn append(&mut self, other: BTreeSet<T>) {
            self.guard.update_tables(Append { other })
        }

        pub fn retain<F>(&mut self, f: F)
        where
            F: 'static + Send + Clone + FnMut(&T) -> bool,
        {
            self.guard.update_tables(Retain { f })
        }
    }
}

#[cfg(test)]
mod lockless_test {
    use super::*;
    use crate::assert_tables_eq;
    use maplit::*;

    #[test]
    fn insert() {
        let expected = btreeset! {
            "hello",
            "world",
        };

        let table = lockless::AsLockHandle::<&str>::default();
        {
            let mut wg = table.write();
            wg.insert("hello");
            wg.insert("world");
            assert_eq!(*wg, expected);
        }
        assert_tables_eq!(table, expected);
    }

    #[test]
    fn clear() {
        let table = lockless::AsLockHandle::<&str>::default();
        {
            let mut wg = table.write();
            wg.insert("hello");
            wg.insert("world");
            wg.clear();
        }

        assert!(table.read().is_empty());
        assert!(table.write().is_empty());
        assert!(table.read().is_empty());
    }

    #[test]
    fn remove_and_take() {
        let expected = btreeset! {
            "hello",
        };
        let table = lockless::AsLockHandle::<&str>::default();
        {
            let mut wg = table.write();
            wg.insert("hello");
            wg.insert("world");
            wg.insert("I");
            assert!(wg.remove("world"));
            assert_eq!(wg.take("I"), Some("I"));
            assert_eq!(wg.take("I"), None);
            assert_eq!(*wg, expected);
        }
        assert_tables_eq!(table, expected);
    }

    #[test]
    fn append() {
        let expected = btreeset! {
            "hello",
            "world",
            "name's",
            "joe",
        };

        let table = lockless::AsLockHandle::<&str>::default();
        {
            let map1 = btreeset! {
                "hello",
                "world",
            };
            let map2 = btreeset! {
                "name's" ,
                "joe" ,
            };
            let mut wg = table.write();
            wg.append(map1);
            wg.append(map2);
            assert_eq!(*wg, expected);
        }
        assert_tables_eq!(table, expected);
    }

    #[test]
    fn retain() {
        let table = lockless::AsLockHandle::new(btreeset! {
            "hello",
            "world",
            "name's",
            "joe",
        });
        table.write().retain(|t| t == &"hello");
        assert_tables_eq!(
            table,
            btreeset! {
                "hello",
            }
        );
    }

    #[test]
    fn debug_str() {
        let table = lockless::AsLockHandle::<i32>::default();
        {
            table.write().insert(12);
        }

        assert_eq!(
            format!("{:?}", table),
            "AsLockHandle { num_readers: 1, num_ops_to_replay: 1, standby_table: {}, active_table: {12} }"
        );
        assert_eq!(
            format!("{:?}", table.write()),
            "AsLockWriteGuard { num_readers: 1, ops_to_replay: 0, standby_table: {12} }",
        );
        assert_eq!(format!("{:?}", table.read()), "{12}",);
    }
}

#[cfg(test)]
mod sync_test {
    use super::*;
    use crate::assert_tables_eq;
    use maplit::*;
    use std::sync::Arc;

    #[test]
    fn insert() {
        let expected = btreeset! {
            "hello",
            "world",
        };

        let table = Arc::new(sync::AsLock::<&str>::default());
        {
            let mut wg = table.write();
            wg.insert("hello");
            wg.insert("world");
            assert_eq!(*wg, expected);
        }
        assert_tables_eq!(table, expected);
    }

    #[test]
    fn clear() {
        let table = sync::AsLock::<&str>::default();
        {
            let mut wg = table.write();
            wg.insert("hello");
            wg.insert("world");
            wg.clear();
        }

        assert!(table.read().is_empty());
        assert!(table.write().is_empty());
        assert!(table.read().is_empty());
    }

    #[test]
    fn remove_and_take() {
        let expected = btreeset! {
            "hello",
        };
        let table = sync::AsLock::<&str>::default();
        {
            let mut wg = table.write();
            wg.insert("hello");
            wg.insert("world");
            wg.insert("I");
            assert!(wg.remove("world"));
            assert_eq!(wg.take("I"), Some("I"));
            assert_eq!(wg.take("I"), None);
            assert_eq!(*wg, expected);
        }
        assert_tables_eq!(table, expected);
    }

    #[test]
    fn append() {
        let expected = btreeset! {
            "hello",
            "world",
            "name's",
            "joe",
        };

        let table = sync::AsLock::<&str>::default();
        {
            let map1 = btreeset! {
                "hello",
                "world",
            };
            let map2 = btreeset! {
                "name's" ,
                "joe" ,
            };
            let mut wg = table.write();
            wg.append(map1);
            wg.append(map2);
            assert_eq!(*wg, expected);
        }
        assert_tables_eq!(table, expected);
    }

    #[test]
    fn retain() {
        let table = sync::AsLock::new(btreeset! {
            "hello",
            "world",
            "name's",
            "joe",
        });
        table.write().retain(|t| t == &"hello");
        assert_tables_eq!(
            table,
            btreeset! {
                "hello",
            }
        );
    }

    #[test]
    fn debug_str() {
        let table = sync::AsLock::<i32>::default();
        {
            table.write().insert(12);
        }

        assert_eq!(
            format!("{:?}", table),
            "AsLock { num_ops_to_replay: 1, standby_table: {12}, active_table: {12} }",
        );
        assert_eq!(
            format!("{:?}", table.write()),
            "AsLockWriteGuard { num_ops_to_replay: 0, standby_table: {12} }",
        );
        assert_eq!(format!("{:?}", table.read()), "{12}",);
    }
}
