
use std::thread;

fn main() {
    println!("Hello, world!");
}

extern "C" {
    fn racy_fn();
}

#[test]
fn test_C_counter() {
    thread::spawn( || {
        unsafe{
            racy_fn();
        }
        
    });
        unsafe {
            racy_fn();
        }

    0
}