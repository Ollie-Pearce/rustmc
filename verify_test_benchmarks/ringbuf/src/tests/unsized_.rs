use super::Rb;
use crate::{
    storage::{Array, Slice},
    traits::*,
};
#[cfg(feature = "alloc")]
use alloc::boxed::Box;

#[no_mangle]
#[test]
fn capacity_2() {
    const CAP: usize = 13;
    let mut rb = Rb::<Array<i32, CAP>>::default();
    let urb = &mut rb as &mut Rb<Slice<i32>>;
    assert_eq!(urb.capacity().get(), CAP);
}

#[no_mangle]
#[test]
fn push_pop_1() {
    let mut rb = Rb::<Array<i32, 1>>::default();
    let urb = &mut rb as &mut Rb<Slice<i32>>;
    let (mut prod, mut cons) = urb.split_ref();

    assert_eq!(prod.try_push(123), Ok(()));
    assert_eq!(prod.try_push(321), Err(321));
    assert_eq!(cons.try_pop(), Some(123));
    assert_eq!(cons.try_pop(), None);
}

#[cfg(feature = "alloc")]
#[no_mangle]
#[test]
fn split_1() {
    let rb = Rb::<Array<i32, 1>>::default();
    let urb = Box::new(rb) as Box<Rb<Slice<i32>>>;
    let (mut prod, mut cons) = urb.split();

    assert_eq!(prod.try_push(123), Ok(()));
    assert_eq!(prod.try_push(321), Err(321));
    assert_eq!(cons.try_pop(), Some(123));
    assert_eq!(cons.try_pop(), None);
}
