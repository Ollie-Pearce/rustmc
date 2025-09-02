#[cfg(test)]
mod examples {
    use scc::HashIndex;
    use std::sync::Arc;
    use std::thread;

#[no_mangle]
    #[test]
    fn single_threaded_4() {
        let workload_size = 256;
        let hashindex: HashIndex<isize, isize> = HashIndex::new();
        for i in 1..workload_size {
            if i % 2 == 0 {
                assert!(hashindex.insert(-i, i).is_ok());
            } else {
                assert!(hashindex.insert(i, i).is_ok());
            }
        }
        for i in 1..workload_size {
            if i % 2 == 0 {
                assert!(hashindex.peek_with(&i, |_, _| ()).is_none());
                assert!(hashindex.peek_with(&-i, |_, _| ()).is_some());
            } else {
                assert!(hashindex.peek_with(&i, |_, _| ()).is_some());
                assert!(hashindex.peek_with(&-i, |_, _| ()).is_none());
            }
        }
        for i in 1..workload_size {
            if i % 2 == 0 {
                assert!(!hashindex.remove(&i));
                assert!(hashindex.remove(&-i));
                assert!(!hashindex.remove(&-i));
            } else {
                assert!(!hashindex.remove(&-i));
                assert!(hashindex.remove(&i));
                assert!(!hashindex.remove(&i));
            }
        }
        assert!(hashindex.is_empty());
    }

#[no_mangle]
    #[test]
    fn multi_threaded_2() {
        let workload_size = 256;
        let hashindex: Arc<HashIndex<isize, isize>> = Arc::default();

        thread::scope(|s| {
            s.spawn(|| {
                for i in 1..workload_size {
                    assert!(hashindex.insert(i, i).is_ok());
                }
                assert!(hashindex.peek_with(&0, |_, _| ()).is_none());
                for i in 1..workload_size {
                    assert!(hashindex.peek_with(&i, |_, _| ()).is_some());
                }
                for i in 1..workload_size {
                    assert!(hashindex.remove(&i));
                }
            });
            s.spawn(|| {
                for i in 1..workload_size {
                    assert!(hashindex.insert(-i, i).is_ok());
                }
                assert!(hashindex.peek_with(&0, |_, _| ()).is_none());
                for i in 1..workload_size {
                    assert!(hashindex.peek_with(&-i, |_, _| ()).is_some());
                }
                for i in 1..workload_size {
                    assert!(hashindex.remove(&-i));
                }
            });
        });

        assert!(hashindex.is_empty());
    }
}
