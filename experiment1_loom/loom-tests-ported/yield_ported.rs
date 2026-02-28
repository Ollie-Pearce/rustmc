// Ported from loom yield.rs


use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::Arc;
use std::thread;

#[no_mangle]
fn yield_completes() {
    let inc = Arc::new(AtomicUsize::new(0));

    {
        let inc = inc.clone();
        thread::spawn(move || {
            inc.store(1, Relaxed);
        });
    }

    loop {
        if 1 == inc.load(Relaxed) {
            return;
        }

        thread::yield_now();
    }
}

fn main() {
    yield_completes();
    println!("Test completed");
}
