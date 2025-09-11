#![allow(clippy::module_name_repetitions)]
use core::{
    marker::PhantomData,
    ptr::{addr_of, addr_of_mut},
    sync::atomic::Ordering,
};

use crate::{Rose, RoseInner};
/// An borrowed iterator over the elements of a `Rose`.
pub struct Iter<'a, T: 'a> {
    rose: PhantomData<&'a Rose<T>>,
    pos: usize,
    ptr: *const RoseInner<T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr.is_null() {
            return None;
        }
        // SAFETY: ptr is a valid pointer if not null.
        let mut length = unsafe { (*self.ptr).length.load(Ordering::Acquire) };
        if length == self.pos {
            // Move on to the next node, we've run out here.
            // SAFETY: ptr is a valid pointer if not null.
            self.ptr = unsafe { (*self.ptr).next };
            if self.ptr.is_null() {
                return None;
            }
            self.pos = 0;
            // SAFETY: ptr is a valid pointer if not null.
            length = unsafe { (*self.ptr).length.load(Ordering::Acquire) };
        }

        // SAFETY: ptr is a valid pointer if not null.
        let base_ptr = unsafe { addr_of!((*self.ptr).data).cast::<T>() };
        let index = length - self.pos - 1;
        self.pos += 1;

        // SAFETY: index is a value less than the length, so it is valid to index to
        Some(unsafe { &*base_ptr.add(index) })
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.ptr.is_null() {
            return (0, Some(0));
        }
        let ptr = self.ptr;
        // SAFETY: ptr is a valid pointer if not null.
        let length = unsafe { (*ptr).length.load(Ordering::Acquire) };
        // SAFETY: ptr is a valid pointer if not null.
        let capacity = unsafe { (*ptr).capacity };
        let total_capacity = (capacity + 1).next_power_of_two() - 1;
        let len = total_capacity - capacity + length;
        (len, Some(len))
    }
}

impl<'a, T> IntoIterator for &'a Rose<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            rose: PhantomData,
            pos: 0,
            ptr: self.last.load(Ordering::Acquire),
        }
    }
}
/// An iterator over the elements of a `Rose` that allows modification.
pub struct IterMut<'a, T: 'a> {
    rose: PhantomData<&'a mut Rose<T>>,
    pos: usize,
    ptr: *mut RoseInner<T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr.is_null() {
            return None;
        }

        // SAFETY: ptr is a valid pointer if not null.
        let mut length = unsafe { (*self.ptr).length.load(Ordering::Acquire) };
        if length == self.pos {
            // Move on to the next node, we've run out here.
            // SAFETY: ptr is a valid pointer if not null.
            self.ptr = unsafe { (*self.ptr).next };
            if self.ptr.is_null() {
                return None;
            }
            self.pos = 0;
            // SAFETY: ptr is a valid pointer if not null.
            length = unsafe { (*self.ptr).length.load(Ordering::Acquire) };
        }

        // SAFETY: ptr is a valid pointer if not null.
        let base_ptr = unsafe { addr_of_mut!((*self.ptr).data).cast::<T>() };
        let index = length - self.pos - 1;
        self.pos += 1;

        // SAFETY: index is a value less than the length, so it is valid to index to
        Some(unsafe { &mut *base_ptr.add(index) })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.ptr.is_null() {
            return (0, Some(0));
        }
        let ptr = self.ptr;
        // SAFETY: ptr is a valid pointer if not null.
        let length = unsafe { (*ptr).length.load(Ordering::Acquire) };
        // SAFETY: ptr is a valid pointer if not null.
        let capacity = unsafe { (*ptr).capacity };
        let total_capacity = (capacity + 1).next_power_of_two() - 1;
        let len = total_capacity - capacity + length;
        (len, Some(len))
    }
}

impl<'a, T> IntoIterator for &'a mut Rose<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut {
            rose: PhantomData,
            pos: 0,
            ptr: self.last.load(Ordering::Acquire),
        }
    }
}

/// An iterator over the elements of a `Rose`.
pub struct IntoIter<T> {
    rose: Rose<T>,
}

impl<T> IntoIterator for Rose<T> {
    type Item = T;

    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { rose: self }
    }
}
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.rose.pop()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.rose.len();
        (len, Some(len))
    }
}
