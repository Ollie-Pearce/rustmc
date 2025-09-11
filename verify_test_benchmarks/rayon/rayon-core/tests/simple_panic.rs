use rayon_core::join;

#[no_mangle]
#[test]
#[should_panic(expected = "should panic")]
fn simple_panic() {
    join(|| {}, || panic!("should panic"));
}
