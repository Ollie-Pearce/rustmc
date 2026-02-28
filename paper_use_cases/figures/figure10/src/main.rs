#![no_main]
#![feature(start)]
#![feature(thread_spawn_unchecked)]
#![no_builtins]
use std::sync::Arc;
use std::cell::Cell;
use std::thread; 


#[start]
#[no_mangle]
#[inline(always)]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    main();
    0
}

#[no_mangle]
#[inline(always)]
pub extern "C" fn main() -> i32{    
    let foo = Arc::new(MyStruct::new(0));

    let foo_clone1 = foo.clone();
    let foo_clone2 = foo.clone();

    let t1 = thread::spawn(move || {
        foo_clone1.increment();
    });

    foo_clone2.increment();

    t1.join().unwrap();

    let final_value = foo.data.get();
    final_value.try_into().unwrap()
}

struct MyStruct {
    data: Cell<i64>,
}

unsafe impl Send for MyStruct {}
unsafe impl Sync for MyStruct {}


impl MyStruct {
    fn new(value: i64) -> Self {
        MyStruct {
            data: Cell::new(value),
        }
    }

    fn increment(&self) {
        self.data.set(self.data.get() + 1);
    }
}