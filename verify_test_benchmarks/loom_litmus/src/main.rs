use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
use std::thread;

fn main() {
    println!("Hello, world!");
}

#[no_mangle]
#[test]
fn load_buffering() {
    let values = Arc::new(Mutex::new(Vec::new()));
    let values_ = values.clone();
    let x = Arc::new(AtomicUsize::new(0));
    let y = Arc::new(AtomicUsize::new(0));

    let th = {
        let (x, y) = (x.clone(), y.clone());
        thread::spawn(move || {
            x.store(y.load(Relaxed), Relaxed);
        })
    };

    let a = x.load(Relaxed);
    y.store(1, Relaxed);

    th.join().unwrap();
    values.lock().unwrap().push(a);
    assert!(values_.lock().unwrap().contains(&1));
}
