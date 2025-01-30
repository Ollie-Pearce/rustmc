use shuttle::sync::Arc;
use shuttle::sync::atomic::AtomicI64;
use shuttle::thread;
use shuttle::sync::atomic::Ordering;


    static CHECKED: AtomicI64 = AtomicI64::new(0);
    static AVAILABLE: AtomicI64 = AtomicI64::new(0);



fn getrand(_buf: &mut [u64]) -> i64 {
    0
}

#[test]
fn buggy_concurrent_inc() {
    shuttle::check_random(|| {
        let handle_1 = thread::spawn(|| {
            is_getrand_available()
        });
    
        let handle_2 = thread::spawn(|| {
            is_getrand_available()
        });
    
        let result_1 = handle_1.join().unwrap();
        let result_2 = handle_2.join().unwrap();
    
        assert_eq!(result_1, result_2);

    });


}

fn is_getrand_available() -> i64 {

    if (CHECKED.load(Ordering::SeqCst) == 0 ){
        let mut buf: [u64; 0] = [];
        let result = getrand(&mut buf);
        let available = if result == -1 { 0 } else { 1 };
        AVAILABLE.store(available, Ordering::SeqCst);
        CHECKED.store(1, Ordering::SeqCst);
        available
    } else {
        AVAILABLE.load(Ordering::SeqCst)
    }
}