use std::thread;
use arcstr::ArcStr;
use std::sync::Arc;
use std::alloc;

    #[test]
    fn cloning_threads() {
            let a = ArcStr::from("abcdefgh");
            let addr = a.as_ptr() as usize;

            let a1 = Arc::new(a);
            let a2 = a1.clone();

            let t1 = thread::spawn(move || {
                let b: ArcStr = (*a1).clone();
                assert_eq!(b.as_ptr() as usize, addr);
            });
            let t2 = thread::spawn(move || {
                let b: ArcStr = (*a2).clone();
                assert_eq!(b.as_ptr() as usize, addr);
            });

            t1.join().unwrap();
            t2.join().unwrap();
    }
    #[test]
    fn drop_timing() {
            let a1 = vec![
                ArcStr::from("s1"),
                ArcStr::from("s2"),
                ArcStr::from("s3"),
                ArcStr::from("s4"),
            ];
            let a2 = a1.clone();

            let t1 = thread::spawn(move || {
                let mut a1 = a1;
                while let Some(s) = a1.pop() {
                    assert!(s.starts_with("s"));
                }
            });
            let t2 = thread::spawn(move || {
                let mut a2 = a2;
                while let Some(s) = a2.pop() {
                    assert!(s.starts_with("s"));
                }
            });

            t1.join().unwrap();
            t2.join().unwrap();
    }

    #[test]
    fn leak_drop() {
            let a1 = ArcStr::from("foo");
            let a2 = a1.clone();

            let t1 = thread::spawn(move || {
                drop(a1);
            });
            let t2 = thread::spawn(move || a2.leak());
            t1.join().unwrap();
            let leaked: &'static str = t2.join().unwrap();
            assert_eq!(leaked, "foo");
    }