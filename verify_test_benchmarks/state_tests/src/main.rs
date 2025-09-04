/*use std::sync::{Arc, RwLock};
use std::cell::Cell;
use std::thread;
use std::sync::Barrier;
use state::*;
use state::init_cell::{InitCell, LocalInitCell};*/

use std::sync::{Arc, RwLock};

fn main() {
    println!("Hello, world!");
}




// Tiny structures to test that dropping works as expected.
struct DroppingStruct(Arc<RwLock<bool>>);
struct DroppingStructWrap(DroppingStruct);

impl Drop for DroppingStruct {
    fn drop(&mut self) {
        *self.0.write().unwrap() = true;
    }
}

// Ensure our DroppingStruct works as intended.
#[no_mangle]
#[test]
fn test_dropping_struct_1() {
    let drop_flag = Arc::new(RwLock::new(false));
    let dropping_struct = DroppingStruct(drop_flag.clone());
    drop(dropping_struct);
    assert_eq!(*drop_flag.read().unwrap(), true);

    let drop_flag = Arc::new(RwLock::new(false));
    let dropping_struct = DroppingStruct(drop_flag.clone());
    let wrapper = DroppingStructWrap(dropping_struct);
    drop(wrapper);
    assert_eq!(*drop_flag.read().unwrap(), true);
}

mod type_map_tests {
    struct DroppingStruct(Arc<RwLock<bool>>);
    struct DroppingStructWrap(DroppingStruct);

    impl Drop for DroppingStruct {
        fn drop(&mut self) {
            *self.0.write().unwrap() = true;
        }
    }
    use state::TypeMap;
    use std::sync::{Arc, RwLock};
    use std::thread;

    // We use one `TYPE_MAP` to get an implicit test since each `test` runs in
    // a different thread. This means we have to `set` different types in each
    // test if we want the `set` to succeed.
    static TYPE_MAP: TypeMap![Send + Sync] = <TypeMap![Send + Sync]>::new();

#[no_mangle]
    #[test]
    fn simple_set_get_1() {
        assert!(TYPE_MAP.set(1u32));
        assert_eq!(*TYPE_MAP.get::<u32>(), 1);
    }

#[no_mangle]
    #[test]
    fn dst_set_get_1_1() {
        assert!(TYPE_MAP.set::<[u32; 4]>([1, 2, 3, 4u32]));
        assert_eq!(*TYPE_MAP.get::<[u32; 4]>(), [1, 2, 3, 4]);
    }

#[no_mangle]
    #[test]
    fn set_get_remote_1() {
        thread::spawn(|| {
            TYPE_MAP.set(10isize);
        }).join().unwrap();

        assert_eq!(*TYPE_MAP.get::<isize>(), 10);
    }

#[no_mangle]
    #[test]
    fn two_put_get_1() {
        assert!(TYPE_MAP.set("Hello, world!".to_string()));

        let s_old = TYPE_MAP.get::<String>();
        assert_eq!(s_old, "Hello, world!");

        assert!(!TYPE_MAP.set::<String>("Bye bye!".into()));
        assert_eq!(TYPE_MAP.get::<String>(), "Hello, world!");
        assert_eq!(TYPE_MAP.get::<String>(), s_old);
    }

#[no_mangle]
    #[test]
    fn many_puts_only_one_succeeds_1_1() {
        let mut threads = vec![];
        for _ in 0..1000 {
            threads.push(thread::spawn(|| {
                TYPE_MAP.set(10i64)
            }))
        }

        let results: Vec<bool> = threads.into_iter().map(|t| t.join().unwrap()).collect();
        assert_eq!(results.into_iter().filter(|&b| b).count(), 1);
        assert_eq!(*TYPE_MAP.get::<i64>(), 10);
    }

    // Ensure setting when already set doesn't cause a drop.
#[no_mangle]
    #[test]
    fn test_no_drop_on_set_1() {
        let drop_flag = Arc::new(RwLock::new(false));
        let dropping_struct = DroppingStruct(drop_flag.clone());

        let _drop_flag_ignore = Arc::new(RwLock::new(false));
        let _dropping_struct_ignore = DroppingStruct(_drop_flag_ignore.clone());

        TYPE_MAP.set::<DroppingStruct>(dropping_struct);
        assert!(!TYPE_MAP.set::<DroppingStruct>(_dropping_struct_ignore));
        assert_eq!(*drop_flag.read().unwrap(), false);
    }

    // Ensure dropping a type_map drops its contents.
#[no_mangle]
    #[test]
    fn drop_inners_on_drop_1_1() {
        let drop_flag_a = Arc::new(RwLock::new(false));
        let dropping_struct_a = DroppingStruct(drop_flag_a.clone());

        let drop_flag_b = Arc::new(RwLock::new(false));
        let dropping_struct_b = DroppingStructWrap(DroppingStruct(drop_flag_b.clone()));

        {
            let type_map = <TypeMap![Send + Sync]>::new();
            type_map.set(dropping_struct_a);
            assert_eq!(*drop_flag_a.read().unwrap(), false);

            type_map.set(dropping_struct_b);
            assert_eq!(*drop_flag_a.read().unwrap(), false);
            assert_eq!(*drop_flag_b.read().unwrap(), false);
        }

        assert_eq!(*drop_flag_a.read().unwrap(), true);
        assert_eq!(*drop_flag_b.read().unwrap(), true);
    }
}

mod cell_tests {
    struct DroppingStruct(Arc<RwLock<bool>>);
struct DroppingStructWrap(DroppingStruct);

impl Drop for DroppingStruct {
    fn drop(&mut self) {
        *self.0.write().unwrap() = true;
    }
}
    use state::InitCell;
    use std::sync::{Arc, RwLock};
    use std::thread;

#[no_mangle]
    #[test]
    fn simple_put_get_1_1() {
        static CELL: InitCell<u32> = InitCell::new();

        assert!(CELL.set(10));
        assert_eq!(*CELL.get(), 10);
    }

#[no_mangle]
    #[test]
    fn no_double_put_1_1() {
        static CELL: InitCell<u32> = InitCell::new();

        assert!(CELL.set(1));
        assert!(!CELL.set(5));
        assert_eq!(*CELL.get(), 1);
    }

#[no_mangle]
    #[test]
    fn many_puts_only_one_succeeds_1_1_1() {
        static CELL: InitCell<u32> = InitCell::new();

        let mut threads = vec![];
        for _ in 0..1000 {
            threads.push(thread::spawn(|| {
                let was_set = CELL.set(10);
                assert_eq!(*CELL.get(), 10);
                was_set
            }))
        }

        let results: Vec<bool> = threads.into_iter().map(|t| t.join().unwrap()).collect();
        assert_eq!(results.into_iter().filter(|&b| b).count(), 1);
        assert_eq!(*CELL.get(), 10);
    }

#[no_mangle]
    #[test]
    fn dst_set_get_1_1_1() {
        static CELL: InitCell<[u32; 4]> = InitCell::new();

        assert!(CELL.set([1, 2, 3, 4]));
        assert_eq!(*CELL.get(), [1, 2, 3, 4]);
    }

    // Ensure dropping a `InitCell<T>` drops `T`.
#[no_mangle]
    #[test]
    fn drop_inners_on_drop_1_1_1() {
        let drop_flag = Arc::new(RwLock::new(false));
        let dropping_struct = DroppingStruct(drop_flag.clone());

        {
            let cell = InitCell::new();
            assert!(cell.set(dropping_struct));
            assert_eq!(*drop_flag.read().unwrap(), false);
        }

        assert_eq!(*drop_flag.read().unwrap(), true);
    }

#[no_mangle]
    #[test]
    fn clone_1() {
        let cell: InitCell<u32> = InitCell::new();
        assert!(cell.try_get().is_none());

        let cell_clone = cell.clone();
        assert!(cell_clone.try_get().is_none());

        assert!(cell.set(10));
        let cell_clone = cell.clone();
        assert_eq!(*cell_clone.get(), 10);
    }
}
