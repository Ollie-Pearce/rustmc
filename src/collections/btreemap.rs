use crate::UpdateTables;
use std::collections::BTreeMap;

struct Insert<K, V> {
    key: K,
    value: V,
}

impl<'a, K, V> UpdateTables<'a, BTreeMap<K, V>, Option<V>> for Insert<K, V>
where
    K: Ord + Clone,
    V: Clone,
{
    fn apply_first(&mut self, table: &'a mut BTreeMap<K, V>) -> Option<V> {
        table.insert(self.key.clone(), self.value.clone())
    }
    fn apply_second(self, table: &mut BTreeMap<K, V>) {
        // Move the value instead of cloning.
        table.insert(self.key, self.value);
    }
}

struct Append<K, V> {
    other: BTreeMap<K, V>,
}

impl<'a, K, V> UpdateTables<'a, BTreeMap<K, V>, ()> for Append<K, V>
where
    K: Ord + Clone,
    V: Clone,
{
    fn apply_first(&mut self, table: &'a mut BTreeMap<K, V>) {
        for (k, v) in self.other.iter() {
            table.insert(k.clone(), v.clone());
        }
    }
    fn apply_second(mut self, table: &mut BTreeMap<K, V>) {
        table.append(&mut self.other)
    }
}

struct Retain<F> {
    f: F,
}

impl<'a, K, V, F> UpdateTables<'a, BTreeMap<K, V>, ()> for Retain<F>
where
    K: Ord,
    V: Clone,
    F: Clone + FnMut(&K, &mut V) -> bool,
{
    fn apply_first(&mut self, table: &'a mut BTreeMap<K, V>) {
        table.retain(self.f.clone())
    }

    fn apply_second(self, table: &mut BTreeMap<K, V>) {
        table.retain(self.f);
    }
}

/// Implementation of BtreeeMap for use in the active_standby model.
/// `lockless::AsLockHandle<K, V>`, should function similarly to
/// `Arc<RwLock<BTreeMap<K, V>>>`.
pub mod lockless {
    use super::*;
    crate::generate_lockless_aslockhandle!(BTreeMap<K, V>);

    impl<'w, K, V> AsLockWriteGuard<'w, K, V>
    where
        K: 'static + Ord + Clone + Send,
        V: 'static + Clone + Send,
    {
        pub fn insert(&mut self, key: K, value: V) -> Option<V> {
            self.guard.update_tables(Insert { key, value })
        }

        pub fn clear(&mut self) {
            self.guard.update_tables_closure(move |table| table.clear())
        }

        pub fn remove<Q>(&mut self, key_like: Q) -> Option<V>
        where
            K: Ord + std::borrow::Borrow<Q>,
            Q: 'static + Ord + Send,
        {
            self.guard
                .update_tables_closure(move |table| table.remove(&key_like))
        }

        pub fn remove_entry<Q>(&mut self, key_like: Q) -> Option<(K, V)>
        where
            K: std::borrow::Borrow<Q>,
            Q: 'static + Ord + Send,
        {
            self.guard
                .update_tables_closure(move |table| table.remove_entry(&key_like))
        }

        pub fn append(&mut self, other: BTreeMap<K, V>) {
            self.guard.update_tables(Append { other })
        }

        pub fn retain<F>(&mut self, f: F)
        where
            F: 'static + Send + Clone + FnMut(&K, &mut V) -> bool,
        {
            self.guard.update_tables(Retain { f })
        }
    }
}

/// Implementation of BtreeeMap for use in the active_standby model.
/// `sync::AsLock<K, V>`, should function similarly to `RwLock<BTreeMap<K,
/// V>>`.
pub mod sync {
    use super::*;
    crate::generate_sync_aslock!(BTreeMap<K, V>);

    impl<'w, K, V> AsLockWriteGuard<'w, K, V>
    where
        K: 'static + Ord + Clone + Send,
        V: 'static + Clone + Send,
    {
        pub fn insert(&mut self, key: K, value: V) -> Option<V> {
            self.guard.update_tables(Insert { key, value })
        }

        pub fn clear(&mut self) {
            self.guard.update_tables_closure(move |table| table.clear())
        }

        pub fn remove<Q>(&mut self, key_like: Q) -> Option<V>
        where
            K: Ord + std::borrow::Borrow<Q>,
            Q: 'static + Ord + Send,
        {
            self.guard
                .update_tables_closure(move |table| table.remove(&key_like))
        }

        pub fn remove_entry<Q>(&mut self, key_like: Q) -> Option<(K, V)>
        where
            K: std::borrow::Borrow<Q>,
            Q: 'static + Ord + Send,
        {
            self.guard
                .update_tables_closure(move |table| table.remove_entry(&key_like))
        }

        pub fn append(&mut self, other: BTreeMap<K, V>) {
            self.guard.update_tables(Append { other })
        }

        pub fn retain<F>(&mut self, f: F)
        where
            F: 'static + Send + Clone + FnMut(&K, &mut V) -> bool,
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
        let expected = btreemap! {
            "hello" => 1,
            "world" => 2,
        };

        let table = lockless::AsLockHandle::<&str, i32>::default();
        {
            let mut wg = table.write();
            wg.insert("hello", 1);
            wg.insert("world", 2);
            assert_eq!(*wg, expected);
        }
        assert_tables_eq!(table, expected);
    }

    #[test]
    fn clear() {
        let table = lockless::AsLockHandle::<&str, i32>::default();
        {
            let mut wg = table.write();
            wg.insert("hello", 1);
            wg.insert("world", 2);
            wg.clear();
        }

        assert!(table.read().is_empty());
        assert!(table.write().is_empty());
        assert!(table.read().is_empty());
    }

    #[test]
    fn remove() {
        let expected = btreemap! {
            "hello" => 1,
        };

        let table = lockless::AsLockHandle::<&str, i32>::default();
        {
            let mut wg = table.write();
            wg.insert("hello", 1);
            wg.insert("world", 2);
            assert_eq!(wg.remove("world"), Some(2));
            assert_eq!(*wg, expected);
        }

        assert_tables_eq!(table, expected);
    }

    #[test]
    fn remove_entry() {
        let expected = btreemap! {
            "hello" => 1,
        };

        let table = lockless::AsLockHandle::<&str, i32>::default();
        {
            let mut wg = table.write();
            wg.insert("hello", 1);
            wg.insert("world", 2);
            assert_eq!(wg.remove_entry("world"), Some(("world", 2)));
            assert_eq!(*wg, expected);
        }

        assert_tables_eq!(table, expected);
    }

    #[test]
    fn append() {
        let expected = btreemap! {
            "hello" => 1,
            "world" => 2,
            "name's" => 3,
            "joe" => 4,
        };

        let table = lockless::AsLockHandle::<&str, i32>::default();
        {
            let map1 = btreemap! {
                "hello" => 1,
                "world" => 2,
            };
            let map2 = btreemap! {
                "name's" => 3,
                "joe" => 4,
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
        let table = lockless::AsLockHandle::new(btreemap! {
            "hello" => 1,
            "world" => 2,
            "name's" => 3,
            "joe" => 4,
        });
        table.write().retain(|k, v| k == &"hello" || *v % 2 == 0);
        assert_tables_eq!(
            table,
            btreemap! {
                "hello" => 1,
                "world" => 2,
                "joe" => 4,
            }
        );
    }

    #[test]
    fn debug_str() {
        let table = lockless::AsLockHandle::<i32, i32>::default();
        {
            table.write().insert(12, -1);
        }

        assert_eq!(
            format!("{:?}", table),
            "AsLockHandle { num_readers: 1, num_ops_to_replay: 1, standby_table: {}, active_table: {12: -1} }",
        );
        assert_eq!(
            format!("{:?}", table.write()),
            "AsLockWriteGuard { num_readers: 1, ops_to_replay: 0, standby_table: {12: -1} }",
        );
        assert_eq!(format!("{:?}", table.read()), "{12: -1}",);
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
        let expected = btreemap! {
            "hello" => 1,
            "world" => 2,
        };

        let table = Arc::new(sync::AsLock::<&str, i32>::default());
        {
            let mut wg = table.write();
            wg.insert("hello", 1);
            wg.insert("world", 2);
            assert_eq!(*wg, expected);
        }

        assert_tables_eq!(table, expected);
    }

    #[test]
    fn clear() {
        let table = Arc::new(sync::AsLock::<&str, i32>::default());
        {
            let mut wg = table.write();
            wg.insert("hello", 1);
            wg.insert("world", 2);
            wg.clear();
        }

        assert!(table.read().is_empty());
        assert!(table.write().is_empty());
        assert!(table.read().is_empty());
    }

    #[test]
    fn remove() {
        let expected = btreemap! {
            "hello" => 1,
        };

        let table = Arc::new(sync::AsLock::<&str, i32>::default());
        {
            let mut wg = table.write();
            wg.insert("hello", 1);
            wg.insert("world", 2);
            assert_eq!(wg.remove("world"), Some(2));
            assert_eq!(*wg, expected);
        }

        assert_tables_eq!(table, expected);
    }

    #[test]
    fn remove_entry() {
        let expected = btreemap! {
            "hello" => 1,
        };

        let table = Arc::new(sync::AsLock::<&str, i32>::default());
        {
            let mut wg = table.write();
            wg.insert("hello", 1);
            wg.insert("world", 2);
            assert_eq!(wg.remove_entry("world"), Some(("world", 2)));
            assert_eq!(*wg, expected);
        }

        assert_tables_eq!(table, expected);
    }

    #[test]
    fn append() {
        let expected = btreemap! {
            "hello" => 1,
            "world" => 2,
            "name's" => 3,
            "joe" => 4,
        };

        let table = Arc::new(sync::AsLock::<&str, i32>::default());
        {
            let map1 = btreemap! {
                "hello" => 1,
                "world" => 2,
            };
            let map2 = btreemap! {
                "name's" => 3,
                "joe" => 4,
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
        let table = sync::AsLock::new(btreemap! {
            "hello" => 1,
            "world" => 2,
            "name's" => 3,
            "joe" => 4,
        });
        table.write().retain(|k, v| k == &"hello" || *v % 2 == 0);
        assert_tables_eq!(
            table,
            btreemap! {
                "hello" => 1,
                "world" => 2,
                "joe" => 4,
            }
        );
    }

    #[test]
    fn debug_str() {
        let table = Arc::new(sync::AsLock::<i32, i32>::default());
        {
            table.write().insert(12, -1);
        }

        assert_eq!(
            format!("{:?}", table),
            "AsLock { num_ops_to_replay: 1, standby_table: {12: -1}, active_table: {12: -1} }",
        );
        assert_eq!(
            format!("{:?}", table.write()),
            "AsLockWriteGuard { num_ops_to_replay: 0, standby_table: {12: -1} }",
        );
        assert_eq!(format!("{:?}", table.read()), "{12: -1}",);
    }
}
