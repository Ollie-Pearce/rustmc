//! This library provides the `Rose` type, a data structure with stable pointers.
//!
//! It is also concurrent and lock-free. The concurrency was a secondary goal of this project, but
//! you can't have a safe API that is useful without Atomics, and it is more useful regardless.
//!
//! # Example
//! ```
//! use rose_bloom::Rose;
//!
//! let rose = Rose::new();
//! let out1 = rose.push(1);
//! rose.push(2);
//! rose.push(3);
//!
//! println!("{out1}"); // 1
//! ```
//! Compare this to the same code with a [Vec], which does not compile and should not compile:
//! ```compile_fail
//! let mut vect = Vec::new();
//! vect.push(1);
//! let out1 = &vect[0];
//! vect.push(2);
//! vect.push(3);
//!
//! println!("{out1}");
//! ```
//!
#![deny(missing_docs)]
#![forbid(unsafe_op_in_unsafe_fn)]
#![warn(clippy::undocumented_unsafe_blocks)]
#![no_std]
use core::{
    alloc::Layout,
    cmp,
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
    mem::{self, ManuallyDrop},
    ops::{Index, IndexMut},
    ptr::{self, addr_of, addr_of_mut, NonNull},
    sync::atomic::{AtomicPtr, AtomicUsize, Ordering},
};
/// Iterator types.
pub mod iter;

extern crate alloc;
use alloc::{
    alloc::{alloc, dealloc, handle_alloc_error},
    vec::Vec,
};
/// A lock-free growing element size linked list with stable pointers.
/// ```
/// use rose_bloom::Rose;
/// let rose = Rose::new();
/// let v = rose.push(1);
/// let _ = rose.push(2);
/// let _ = rose.push(3);
/// assert_eq!(rose[0], 3);
/// assert_eq!(rose[1], 2);
/// println!("{}", v); // prints 1
/// assert_eq!(rose[2], 1);
/// ```
pub struct Rose<T> {
    last: AtomicPtr<RoseInner<T>>,
}

#[repr(C)]
struct RoseInner<T> {
    /// The next node in the list.
    next: *mut RoseInner<T>,
    // The number of valid elements in the array. It is less than or equal to the capacity and
    //  is a power of two.
    length: AtomicUsize,
    /// The capacity is the number of elements that can be stored in the array. It is always
    /// greater than or equal to one and is a power of two.
    capacity: usize,
    /// The number of writers currently writing to the array. It is always less than or equal
    /// to length.
    writers: AtomicUsize,
    /// The active index. When `writers` is zero, this should be the same as the length.
    active_index: AtomicUsize,
    /// The data. This is a ZST, but you can peer off the end of the struct into the elements.
    data: [T; 0],
}

const fn next_multiple_of(n: usize, m: usize) -> usize {
    match n % m {
        0 => n,
        r => n + (m - r),
    }
}

impl<T> Rose<T> {
    const ALIGNMEMT: usize = mem::align_of::<RoseInner<T>>();

    #[track_caller]
    const fn size_assert(cap: usize) {
        assert!(next_multiple_of(Self::size(cap), Self::ALIGNMEMT) <= isize::MAX as usize);
    }

    const fn size(cap: usize) -> usize {
        mem::size_of::<RoseInner<T>>().saturating_add(mem::size_of::<T>().saturating_mul(cap))
    }
    const fn align() -> usize {
        mem::align_of::<RoseInner<T>>()
    }
    const fn layout(cap: usize) -> Layout {
        let size = Self::size(cap);
        Self::size_assert(size);
        // SAFETY: align_of returns a power of 2, and the size has been checked to be less than
        // isize::MAX.
        unsafe { Layout::from_size_align_unchecked(size, Self::ALIGNMEMT) }
    }

    fn alloc(capacity: usize) -> NonNull<RoseInner<T>> {
        // These could only be the case if someone is trying to allocate a rose with a never type,
        // which is incredibly stupid. But welcome to checking all invariants.
        // I also am not convinced that it would happen with repr(C) anyway, but it's better to be
        // safe than sorry.
        // These should both compile out if they're false because they're all constants.
        assert_ne!(Self::align(), 0);
        assert!(Self::size(1) != 0);
        let layout = Self::layout(capacity);
        // SAFETY: Size is not zero, because RoseInner<T> is at least size_of::<usize>() * 4, so
        // we can alloc.
        // We check for a null pointer and handle the alloc error, so it must be nonnull.
        unsafe {
            let memory = alloc(layout);
            if memory.is_null() {
                handle_alloc_error(layout)
            }
            NonNull::new_unchecked(memory.cast::<RoseInner<T>>())
        }
    }
    /// Creates a new [`Rose<T>`].
    /// ```
    /// # use rose_bloom::Rose;
    /// let rose: Rose<i32> = Rose::new();
    /// ```
    #[must_use]
    pub fn new() -> Rose<T> {
        // Allocation code
        let memory = Self::alloc(1).as_ptr();
        // Initialization code
        // SAFETY: We are the only ones who can access this memory, so we can initialize it.
        unsafe {
            addr_of_mut!((*memory).next).write(ptr::null_mut());
            addr_of_mut!((*memory).length).write(AtomicUsize::new(0));
            addr_of_mut!((*memory).capacity).write(1);
            addr_of_mut!((*memory).writers).write(AtomicUsize::new(0));
            addr_of_mut!((*memory).active_index).write(AtomicUsize::new(0));
        };
        Rose {
            last: AtomicPtr::new(memory),
        }
    }

    /// Push a `value` into the [`Rose`].
    /// ```
    /// # use rose_bloom::Rose;
    /// let rose = Rose::new();
    /// rose.push(1);
    /// ```
    ///
    /// This is a O(1) operation.
    /// # Panics
    /// Panics if there are `usize::MAX` concurrent writes happening at once.
    pub fn push(&self, value: T) -> &T {
        let mut item = ManuallyDrop::new(value);
        loop {
            let last = self.last.load(Ordering::Acquire);

            // Both of these are from the same `RoseInner`
            // SAFETY: Last is always a valid pointer.
            let length = unsafe { &(*last).length };
            // SAFETY: Last is always a valid pointer.
            let capacity = unsafe { (*last).capacity };

            let index = length.fetch_update(Ordering::AcqRel, Ordering::Acquire, |x| {
                if x == capacity {
                    // We need to allocate more.
                    None
                } else {
                    Some(x + 1)
                }
            });

            if let Ok(len) = index {
                // SAFETY: last is a valid pointer.
                // We use AcqRel to ensure other threads know we're writing.
                // `writers` cannot overflow because it's always less than or equal to `capacity`.
                unsafe { (*last).writers.fetch_add(1, Ordering::AcqRel) };

                // SAFETY: We now "own" this index because we increased the length. This ownership
                // permits us to write to it.
                unsafe {
                    let out = addr_of_mut!((*last).data).cast::<T>().add(len);
                    out.write(addr_of!(item).cast::<T>().read());

                    // Load length to get a better estimate of how long this thing is.
                    let new_length = length.load(Ordering::Acquire);

                    // Because of the AcqRel ordering, this will be a unique number for us.
                    let writers = (*last).writers.fetch_sub(1, Ordering::AcqRel) - 1;
                    let _ = (*last).active_index.fetch_update(
                        Ordering::AcqRel,
                        Ordering::Acquire,
                        |x| {
                            if writers == 0 {
                                // Or, if we are the last writer, we should set the active index.
                                // This is `new_length` because there is no better guess for what it
                                // should be. Eventually, this will be fixed by another writer, and
                                // if nothing else, when deallocating, length will be read for
                                // dropping elements.

                                // All methods that have exclusive access will fix up everything
                                // they touch.

                                // This is probably correct though, because it is only wrong in
                                // cases with more than three writers, and when 3 writes happened
                                // in a row for two other operations, but we cannot complete 2, and
                                // those operations occurred in an order such that neither of them
                                // were first.
                                
                                // new_length >= len + 1, so we don't have to do anything special
                                // for the case where we're the final writer and we have no
                                Some(new_length)
                            } else if x == len {
                                // If our index is just past the current active index, we can set the
                                // active index to us.
                                Some(len + 1)
                            } else {
                                None
                            }
                        },
                    );

                    // SAFETY: last is a valid pointer.
                    return &*out;
                }
            }

            let new = Self::alloc(capacity * 2);
            // SAFETY: We just created the pointer, so we can do whatever we want with it.
            let refer = unsafe {
                let new = new.as_ptr();
                addr_of_mut!((*new).next).write(last);
                addr_of_mut!((*new).length).write(AtomicUsize::new(1));
                addr_of_mut!((*new).capacity).write(capacity * 2);
                addr_of_mut!((*new).writers).write(AtomicUsize::new(0));
                addr_of_mut!((*new).active_index).write(AtomicUsize::new(1));
                let out = addr_of_mut!((*new).data).cast::<T>();
                out.write(addr_of_mut!(item).cast::<T>().read());
                &*out
            };
            let success = self
                .last
                .fetch_update(Ordering::AcqRel, Ordering::Acquire, |x| {
                    if x == last {
                        Some(new.as_ptr())
                    } else {
                        // Somebody changed our pointer.
                        None
                    }
                });
            if success.is_ok() {
                return refer;
            }
            // Another thread updated the pointer before us.
            // SAFETY: We created the pointer via `alloc` so we can deallocate it.
            unsafe {
                dealloc(new.as_ptr().cast::<u8>(), Self::layout(capacity * 2));
            };
        }
    }

    /// Returns the length of this [`Rose<T>`].
    ///
    /// This is an O(log n) operation.
    ///
    /// It returns the number of currently readable indexes.
    ///
    /// ```
    /// # use rose_bloom::Rose;
    /// let rose = Rose::new();
    /// rose.push(1);
    /// rose.push(2);
    /// rose.push(3);
    /// assert_eq!(rose.len(), 3);
    /// ```

    pub fn len(&self) -> usize {
        let mut current = self.last.load(Ordering::Acquire);
        let mut out = 0;
        while !current.is_null() {
            // SAFETY: current is a valid pointer.
            let active_index = unsafe { (*current).active_index.load(Ordering::Acquire) };
            // active_index expresses the number of elements that you can read.
            out += active_index;
            // SAFETY: current is a valid pointer
            current = unsafe { (*current).next };
        }
        out
    }

    /// Returns the length.
    ///
    /// This runs in O(1) time.
    ///
    /// It does not account for in-flight writes.
    ///
    /// ```
    /// # use rose_bloom::Rose;
    /// let rose = Rose::new();
    /// rose.push(1);
    /// rose.push(2);
    /// rose.push(3);
    /// assert_eq!(rose.len_fast(), 3);
    /// ```
    pub fn len_fast(&self) -> usize {
        let last = self.last.load(Ordering::Acquire);

        // SAFETY: `last` is a valid pointer.
        unsafe {
            let current_cap = (*last).capacity;
            let current_len = (*last).length.load(Ordering::Acquire);
            self.capacity() - (current_cap - current_len)
        }
    }
    /// Returns the capacity of this [`Rose<T>`].
    ///
    /// O(1) operation.
    ///
    /// ```
    /// # use rose_bloom::Rose;
    /// let rose = Rose::new();
    /// rose.push(1);
    /// rose.push(2);
    /// rose.push(3);
    /// assert!(rose.capacity() >= 3);
    /// ```
    pub fn capacity(&self) -> usize {
        let last = self.last.load(Ordering::Acquire);
        // SAFETY: last is a valid pointer.
        let capacity = unsafe { (*last).capacity };
        (capacity + 1).next_power_of_two() - 1
    }
    /// Are there elements in the [`Rose<T>`]?
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Iterate over the elements of the [`Rose<T>`].
    /// ```
    /// # use rose_bloom::Rose;
    /// let rose = Rose::new();
    /// rose.push(1);
    /// rose.push(2);
    /// rose.push(3);
    /// for i in rose.iter() {
    ///    println!("{}", i);
    /// }
    /// ```
    pub fn iter(&self) -> iter::Iter<'_, T> {
        self.into_iter()
    }
    /// Iterate over the elements of the [`Rose<T>`], allowing modification.
    /// ```
    /// # use rose_bloom::Rose;
    /// let mut rose = Rose::new();
    /// rose.push(1);
    /// rose.push(2);
    /// rose.push(3);
    /// for i in rose.iter_mut() {
    ///    *i += 1;
    /// }
    /// ```
    /// This is an O(n) operation.
    pub fn iter_mut(&mut self) -> iter::IterMut<'_, T> {
        self.into_iter()
    }
    /// Removes the last element from the [`Rose<T>`] and returns it, or `None` if it is empty.
    ///
    /// This is an O(1) operation.
    /// ```
    /// # use rose_bloom::Rose;
    /// let mut rose = Rose::new();
    /// rose.push(1);
    /// rose.push(2);
    /// rose.push(3);
    /// assert_eq!(rose.pop(), Some(3));
    /// assert_eq!(rose.pop(), Some(2));
    /// assert_eq!(rose.pop(), Some(1));
    /// assert_eq!(rose.pop(), None);
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        // If there is no node, return None
        let mut current = NonNull::new(*self.last.get_mut())?;

        // SAFETY: current is a valid pointer
        let length = unsafe { *(*current.as_ptr()).length.get_mut() };
        if length == 0 {
            // Time to deallocate the current node and move on to the next one.

            // SAFETY: current is a valid pointer
            let next = unsafe { (*current.as_ptr()).next };
            // SAFETY: current is a valid pointer
            let capacity = unsafe { (*current.as_ptr()).capacity };
            // SAFETY: self.last has a valid pointer and that is the layout
            // that was allocated with.
            unsafe {
                dealloc(
                    self.last.get_mut().cast::<u8>(),
                    Rose::<T>::layout(capacity),
                );
            }
            self.last = AtomicPtr::new(next);
            // If there is no node, return None
            current = NonNull::new(*self.last.get_mut())?;
        }
        // SAFETY: current is a valid pointer
        debug_assert_ne!(unsafe { *(*current.as_ptr()).length.get_mut() }, 0);
        // SAFETY: Current must have some length, because we either just checked that, or we
        // went forward to the next node.

        // If we went forward to the next node: because we just arrived at this node, it must have
        // at least one element. This is because nodes that are not last always have
        // `capacity == length` and capacity must be at least 1.

        // We have complete control over this datastructure due to the mutable reference, so we can
        // use `.get_mut`
        let index = unsafe {
            *(*current.as_ptr()).length.get_mut() -= 1;
            *(*current.as_ptr()).active_index.get_mut() = *(*current.as_ptr()).length.get_mut();
            *(*current.as_ptr()).length.get_mut()
        };
        // SAFETY: data is a ZST
        let base_ptr = unsafe { addr_of_mut!((*current.as_ptr()).data).cast::<T>() };

        // SAFETY: index is va because
        Some(unsafe { base_ptr.add(index).read() })
    }

    /// Returns a reference to an element, or `None` if the index is out of bounds.
    /// This indexes from the back of the [`Rose<T>`] to the front.
    /// This is an O(lg n) operation.
    /// # Examples
    /// ```
    /// # use rose_bloom::Rose;
    /// let mut rose = Rose::new();
    /// rose.push(1);
    /// rose.push(2);
    /// rose.push(3);
    /// assert_eq!(rose.get(0), Some(&3));
    /// assert_eq!(rose.get(1), Some(&2));
    /// assert_eq!(rose.get(2), Some(&1));
    /// assert_eq!(rose.get(3), None);
    /// ```
    pub fn get(&self, mut index: usize) -> Option<&T> {
        let mut current = self.last.load(Ordering::Acquire);
        while !current.is_null() {
            // SAFETY: the pointer is always valid
            // length - writers is the number of valid writers.
            let length = unsafe { (*current).active_index.load(Ordering::Acquire) };
            if length > index {
                // Great, we found it
                // SAFETY: add is in bounds because length > index
                return Some(unsafe {
                    &*(addr_of!((*current).data)
                        .cast::<T>()
                        .add(length - index - 1))
                });
            }
            // If we didn't find it, go to the next list item.
            index -= length;
            // SAFETY: current is valid
            current = unsafe { (*current).next };
        }
        None
    }
    /// Returns a mutable reference to an element, or `None` if the index is out of bounds.
    /// This indexes from the back of the [`Rose<T>`] to the front.
    /// This is an O(lg n) operation.
    /// # Examples
    /// ```
    /// # use rose_bloom::Rose;
    /// let mut rose = Rose::new();
    /// rose.push(1);
    /// rose.push(2);
    /// rose.push(3);
    /// assert_eq!(rose.get_mut(0), Some(&mut 3));
    /// assert_eq!(rose.get_mut(1), Some(&mut 2));
    /// assert_eq!(rose.get_mut(2), Some(&mut 1));
    /// assert_eq!(rose.get_mut(3), None);
    /// ```
    pub fn get_mut(&mut self, mut index: usize) -> Option<&mut T> {
        let mut current = *self.last.get_mut();
        while !current.is_null() {
            // SAFETY: the pointer is always valid
            // active_index is the number of valid areas
            let length = unsafe { *(*current).active_index.get_mut() };
            if length > index {
                // Great, we found it
                // SAFETY: add is in bounds because length > index
                return Some(unsafe {
                    &mut *(addr_of_mut!((*current).data)
                        .cast::<T>()
                        .add(length - index - 1))
                });
            }
            // If we didn't find it, go to the next list item.
            index -= length;
            // SAFETY: current is valid
            current = unsafe { (*current).next };
        }
        None
    }

    /// Removes an element from the [`Rose<T>`] and returns it, or `None` if it is empty.
    /// This indexes from the back of the [`Rose<T>`] to the front.
    ///
    /// This is an O(n) operation. Avoid using this if possible. It is better to use [`Self::pop`].
    /// # Examples
    /// ```
    /// # use rose_bloom::Rose;
    /// let mut rose = Rose::new();
    /// rose.push(Box::new(1));
    /// rose.push(Box::new(2));
    /// rose.push(Box::new(3));
    /// assert_eq!(rose.remove(2), Some(Box::new(1)));
    /// ```
    pub fn remove(&mut self, mut index: usize) -> Option<T> {
        // This is checked first because we are moving the list along as we go.
        if self.len_fast() <= index {
            // We can't remove anything if the index is out of bounds.
            // len_fast is correct because of our mutable reference.
            return None;
        }
        let mut current = self.last.load(Ordering::Acquire);
        let mut prev: Option<T> = None;
        loop {
            // current is always nonnull because we checked that we are in bounds via length.
            debug_assert!(!current.is_null());

            // SAFETY: the pointer is always valid when nonnull.
            // It is nonnull because we are in bounds in the Rose.
            let length = unsafe { *(*current).length.get_mut() };

            // We have exclusive access, nobody should be writing.
            // SAFETY: current is always valid
            debug_assert_eq!(unsafe { *(*current).writers.get_mut() }, 0);

            if length > index {
                // Great, we found it
                // SAFETY: add is in bounds because length > index, and there
                // are no writers, due to exclusive accesss
                let elem_ptr = unsafe {
                    addr_of_mut!((*current).data)
                        .cast::<T>()
                        .add(length - index - 1)
                };
                // Read the value for returning it.
                // SAFETY: the pointer is valid, and no other copies of it will exist because
                // we overwrite it.
                let out = unsafe { elem_ptr.read() };
                // Move all of the elements over.
                // SAFETY: elem_ptr is valid, and we move over all of the extra elements, which are
                // valid.
                unsafe { elem_ptr.add(1).copy_to(elem_ptr, length - index - 1) };

                // Subtract from `length` because we remove an element. Update `active_index`
                // because it should be `length` if there are no writers.
                // SAFETY: `current` is a valid pointer to which we have exclusive access.
                unsafe {
                    *(*current).length.get_mut() -= 1;
                    *(*current).active_index.get_mut() = *(*current).length.get_mut();
                }

                // If an element came off at a previous iteration, we need to add it to the end here
                if let Some(elem) = prev {
                    // SAFETY: current is valid.
                    unsafe {
                        addr_of_mut!((*current).data)
                            .cast::<T>()
                            .add(*(*current).length.get_mut())
                            .write(elem);
                        // We added an element, so add one to the length
                        *(*current).length.get_mut() += 1;
                        // Update active_index to match.
                        *(*current).active_index.get_mut() = *(*current).length.get_mut();
                    }
                }
                return Some(out);
            }
            // We need to move the list, because we didn't return.

            // Get the base pointer. The rest of the data in this node is stored past it
            // SAFETY: data is valid, because it's a ZST
            let base_ptr = unsafe { addr_of_mut!((*current).data).cast::<T>() };
            let first = if length >= 1 {
                // Keep the first element safe
                // SAFETY: The length is at least 1, so this value is valid.
                let first = Some(unsafe { base_ptr.read() });
                // This doesn't overflow because `length` is always at least 1.
                let elements = length - 1;
                // We need to move `elements` forward 1.
                // SAFETY: add is in bounds because length >= 1, and there are `elements` we need to
                // move because length
                unsafe {
                    base_ptr.add(1).copy_to(base_ptr, elements);
                }
                // We removed an index, subtract 1. Also fixup the active_index as we go along.
                // SAFETY: current is always valid and we have exclusive access to the list,
                // so we can use .get_mut().
                unsafe {
                    *(*current).length.get_mut() -= 1;
                    *(*current).active_index.get_mut() = *(*current).length.get_mut();
                };
                first
            } else {
                None
            };

            // If we had previously removed an element add it back.
            if let Some(elem) = prev {
                // SAFETY:
                unsafe {
                    base_ptr.add(*(*current).length.get_mut()).write(elem);
                    *(*current).length.get_mut() += 1;
                    *(*current).active_index.get_mut() = *(*current).length.get_mut();
                }
            }
            prev = first;
            // If we didn't find it, go to the next list item.
            index -= length;
            // SAFETY: current is a valid pointer, and this will only iterate be iterating if we can
            // read length.
            current = unsafe { (*current).next };
        }
    }

    /// Push an element to the [`Rose<T>`].
    /// This takes `&mut self`, so can be marginally faster than [`Self::push`] and can return `&mut T`.
    ///
    /// This is an O(1) operation, amortized.
    /// # Examples
    /// ```
    /// # use rose_bloom::Rose;
    /// let mut rose = Rose::new();
    /// rose.push_mut(1);
    /// rose.push_mut(2);
    /// rose.push_mut(3);
    /// assert_eq!(rose.len(), 3);
    /// ```
    pub fn push_mut(&mut self, value: T) -> &mut T {
        let current = *self.last.get_mut();
        // SAFETY: current is valid
        let length = unsafe { *(*current).length.get_mut() };
        // SAFETY: current is valid
        let capacity = unsafe { (*current).capacity };
        // We have space.
        if length < capacity {
            // SAFETY: we have space, so we can just write it directly
            unsafe {
                let ptr = addr_of_mut!((*current).data).cast::<T>().add(length);
                ptr.write(value);
                *(*current).active_index.get_mut() = length + 1;
                *(*current).length.get_mut() += 1;
                &mut *ptr
            }
        } else {
            // We need to allocate
            let ptr = Self::alloc(capacity * 2);

            // SAFETY: we own everything so we can do whatever we want.
            unsafe {
                let ptr = ptr.as_ptr();
                addr_of_mut!((*ptr).next).write(current);
                addr_of_mut!((*ptr).length).write(AtomicUsize::new(1));
                addr_of_mut!((*ptr).capacity).write(capacity * 2);
                addr_of_mut!((*ptr).writers).write(AtomicUsize::new(0));
                addr_of_mut!((*ptr).active_index).write(AtomicUsize::new(1));
                addr_of_mut!((*ptr).data).cast::<T>().write(value);
                *self.last.get_mut() = ptr;
                &mut *addr_of_mut!((*ptr).data).cast::<T>()
            }
        }
    }
    /// Returns a reference to the first element of the [`Rose<T>`].
    /// This is an O(lg n) operation.
    /// # Examples
    /// ```
    /// use rose_bloom::Rose;
    /// let mut rose = Rose::new();
    /// rose.push(1);
    /// rose.push(2);
    /// rose.push(3);
    /// assert_eq!(rose.first(), Some(&1));
    /// ```
    pub fn first(&self) -> Option<&T> {
        let mut current = self.last.load(Ordering::Acquire);
        let mut out = None;
        while !current.is_null() {
            // SAFETY: the pointer is always valid
            // active_index is the number of valid places.
            let length = unsafe { (*current).active_index.load(Ordering::Acquire) };
            if length > 0 {
                // SAFETY: read is valid because length > 0
                out = Some(unsafe { &*addr_of!((*current).data).cast::<T>() });
            }
            // SAFETY: current is valid
            current = unsafe { (*current).next };
        }
        out
    }

    /// Returns a mutable reference to the first element of the [`Rose<T>`].
    /// This is an O(lg n) operation.
    /// # Examples
    /// ```
    /// use rose_bloom::Rose;
    /// let mut rose = Rose::new();
    /// rose.push(1);
    /// rose.push(2);
    /// rose.push(3);
    /// assert_eq!(rose.first_mut(), Some(&mut 1));
    /// ```
    pub fn first_mut(&mut self) -> Option<&mut T> {
        let mut current = *self.last.get_mut();
        let mut out = None;
        while !current.is_null() {
            // SAFETY: the pointer is always valid
            // active_index is the number of valid places.
            let length = *unsafe { (*current).active_index.get_mut() };
            if length > 0 {
                // SAFETY: read is valid because length > 0
                out = Some(unsafe { &mut *addr_of_mut!((*current).data).cast::<T>() });
            }
            // SAFETY: current is valid
            current = unsafe { (*current).next };
        }
        out
    }

    /// Returns a reference to the last element of the [`Rose<T>`].
    /// This is an O(1) operation.
    /// # Examples
    /// ```
    /// use rose_bloom::Rose;
    /// let mut rose = Rose::new();
    /// rose.push(1);
    /// rose.push(2);
    /// rose.push(3);
    /// assert_eq!(rose.last(), Some(&3));
    /// ```
    pub fn last(&self) -> Option<&T> {
        self.get(0)
    }

    /// Returns a mutable reference to the last element of the [`Rose<T>`].
    /// This is an O(1) operation.
    /// # Examples
    /// ```
    /// use rose_bloom::Rose;
    /// let mut rose = Rose::new();
    /// rose.push(1);
    /// rose.push(2);
    /// rose.push(3);
    /// assert_eq!(rose.last_mut(), Some(&mut 3));
    /// ```
    pub fn last_mut(&mut self) -> Option<&mut T> {
        self.get_mut(0)
    }

    /// Clears the [`Rose<T>`], removing all values.
    /// This is an O(n) operation.
    /// # Examples
    /// ```
    /// use rose_bloom::Rose;
    /// let mut rose = Rose::new();
    /// rose.push(1);
    /// rose.push(2);
    /// rose.push(3);
    /// assert_eq!(rose.len(), 3);
    /// rose.clear();
    /// assert_eq!(rose.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        let mut current = *self.last.get_mut();
        // SAFETY: current is valid
        let mut capacity = unsafe { (*current).capacity };
        while capacity != 1 {
            // SAFETY: current is valid
            let length = *unsafe { (*current).length.get_mut() };
            // SAFETY: data will have valid data for `length` items
            let slice = unsafe {
                core::slice::from_raw_parts_mut(addr_of_mut!((*current).data).cast::<T>(), length)
            };
            // Drop items
            // SAFETY: slice is a valid slice
            unsafe { ptr::drop_in_place(slice) };
            // SAFETY: current is valid

            let next = unsafe { (*current).next };
            // Dealloc Rose
            // SAFETY: current is a pointer allocated by `alloc`, with that layout.
            unsafe { dealloc(current.cast::<u8>(), Self::layout((*current).capacity)) }
            current = next;
            // SAFETY: current is valid, because we've moved on.
            capacity = unsafe { (*current).capacity };
        }
        // Capacity is 1, so we can just drop the last item, if it exists.
        // SAFETY: current is valid.
        let length = *unsafe { (*current).length.get_mut() };
        if length == 1 {
            // SAFETY: data is valid
            unsafe {
                // Drop data.
                addr_of_mut!((*current).data).cast::<T>().read()
            };
            // SAFETY: current is valid
            *unsafe { (*current).length.get_mut() } = 0;
            // SAFETY: current is valid
            *unsafe { (*current).active_index.get_mut() } = 0;
        }

        self.last = AtomicPtr::new(current);
    }
}

/// This index is the number from the back.
/// This is a O(lg n) operation.
impl<T> Index<usize> for Rose<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match self.get(index) {
            Some(x) => x,
            None => panic!(
                "Index out of bounds, {index} is greater than {len}",
                len = self.len()
            ),
        }
    }
}

/// This index is the number from the back.
/// This is a O(lg n) operation.
impl<T> IndexMut<usize> for Rose<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if let Some(x) = self.get_mut(index) {
            // SAFETY: This transmute is purely to avoid a compiler error that will be fixed with
            // polonius. I confirmed this code without the transmute works when using polonius.
            return unsafe { core::mem::transmute::<&mut T, &mut T>(x) };
        };
        panic!(
            "Index out of bounds, {index} is greater than {len}",
            len = self.len()
        )
    }
}
impl<T> Drop for Rose<T> {
    fn drop(&mut self) {
        let mut current = *self.last.get_mut();
        while !current.is_null() {
            // SAFETY: current is valid, data will have valid data for `length` items
            // We can use get_mut because we have a mutable reference the Rose.
            let length = *unsafe { (*current).length.get_mut() };
            // SAFETY: current is valid
            // the slice is valid because no other mutable references exist because the Rose is
            // dropped, and length is the length of valid data.
            let slice = unsafe {
                core::slice::from_raw_parts_mut(addr_of_mut!((*current).data).cast::<T>(), length)
            };
            // Drop items
            // SAFETY: slice is a valid slice, and nobody will touch those items after this.
            unsafe { ptr::drop_in_place(slice) };
            // SAFETY: current is valid
            let next = unsafe { (*current).next };
            // Dealloc Rose
            // SAFETY: Current is a pointer allocated by the GlobalAlloc, and that layout is what
            // we used to create it.
            unsafe { dealloc(current.cast::<u8>(), Self::layout((*current).capacity)) }

            current = next;
        }
    }
}

impl<T> From<Vec<T>> for Rose<T> {
    fn from(mut vec: Vec<T>) -> Self {
        let rose = Self::new();
        for item in vec.drain(..) {
            rose.push(item);
        }
        rose
    }
}

impl<T> From<Rose<T>> for Vec<T> {
    fn from(rose: Rose<T>) -> Self {
        rose.into_iter().collect()
    }
}

impl<T> Clone for Rose<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        let new = Self::new();
        for x in self {
            new.push(x.clone());
        }
        new
    }
}

impl<T> Debug for Rose<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T> Default for Rose<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Eq for Rose<T> where T: Eq {}
impl<T> PartialEq for Rose<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<T> Hash for Rose<T>
where
    T: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut current = self.last.load(Ordering::Acquire);
        while !current.is_null() {
            // SAFETY: current is valid
            let length = unsafe { (*current).active_index.load(Ordering::Acquire) };
            // SAFETY: Roses are valid for all up to the active_index.
            let slice = unsafe {
                core::slice::from_raw_parts(addr_of_mut!((*current).data).cast::<T>(), length)
            };
            T::hash_slice(slice, state);

            // SAFETY: current is valid
            let next = unsafe { (*current).next };
            current = next;
        }
    }
}

impl<T> Extend<T> for Rose<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push(item);
        }
    }
}

impl<T> FromIterator<T> for Rose<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut rose = Self::new();
        rose.extend(iter);
        rose
    }
}
impl<T> Ord for Rose<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.iter().cmp(other.iter())
    }
}

impl<T> PartialOrd for Rose<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.iter().partial_cmp(other.iter())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

#[no_mangle]
    #[test]
    fn it_works() {
        use alloc::boxed::Box;
        let rose = Rose::new();

        assert!(rose.is_empty());
        rose.push(Box::new(1));
        assert_eq!(rose.len(), 1);
        rose.push(Box::new(2));
        assert_eq!(rose.len(), 2);
        rose.push(Box::new(3));
        assert_eq!(rose.len(), 3);

        assert_eq!(*rose[0], 3);
        assert_eq!(*rose[1], 2);
        assert_eq!(*rose[2], 1);

        let mut iter = (&rose).into_iter();
        assert_eq!(iter.next(), Some(&Box::new(3)));
        assert_eq!(iter.next(), Some(&Box::new(2)));
        assert_eq!(iter.next(), Some(&Box::new(1)));
        assert_eq!(iter.next(), None);

        let mut iter = rose.into_iter();
        assert_eq!(iter.next(), Some(Box::new(3)));
        assert_eq!(iter.next(), Some(Box::new(2)));
        assert_eq!(iter.next(), Some(Box::new(1)));
        assert_eq!(iter.next(), None);
    }

#[no_mangle]
    #[test]
    fn mut_iter() {
        use alloc::boxed::Box;
        let mut rose = Rose::new();
        rose.push(Box::new(1));
        rose.push(Box::new(2));
        rose.push(Box::new(3));
        for x in rose.iter_mut() {
            **x += 1;
        }
        let mut iter = rose.iter_mut();
        assert_eq!(iter.next(), Some(&mut Box::new(4)));
        assert_eq!(iter.next(), Some(&mut Box::new(3)));
        assert_eq!(iter.next(), Some(&mut Box::new(2)));
        assert_eq!(iter.next(), None);
    }
#[no_mangle]
    #[test]
    fn dataraces() {
        extern crate std;
        use std::thread;
        let rose = Rose::new();
        rose.push(0);
        thread::scope(|s| {
            s.spawn(|| {
                let _ = rose[0];
                rose.push(1);
            });
            s.spawn(|| {
                let _ = rose[0];
                rose.push(2);
            });
        });
    }
#[no_mangle]
    #[test]
    #[should_panic]
    fn out_of_bounds() {
        let rose = Rose::new();
        rose.push(0);
        let _ = &rose[1];
    }
#[no_mangle]
    #[test]
    #[should_panic]
    fn out_of_bounds_mut() {
        let mut rose = Rose::new();
        rose.push(0);
        let _ = &mut rose[1];
    }
#[no_mangle]
    #[test]
    fn remove() {
        let mut rose: Rose<i32> = Rose::new();
        assert_eq!(rose.remove(0), None);
    }
#[no_mangle]
    #[test]
    fn first_works_append_order() {
        extern crate std;
        use std::thread;
        let rose = Rose::new();
        rose.push(1);
        thread::scope(|s| {
            s.spawn(|| {
                for x in 2..10 {
                    rose.push(x);
                }
            });
            s.spawn(|| {
                // This should work because we pushed 1 item earlier
                // so no matter how many items we have, there's always _a_ first
                for _ in 0..10 {
                    rose.first().unwrap();
                }
            });
        });

        // into_iter iterates in reverse order
        assert_eq!(
            rose.into_iter().collect::<Vec<_>>(),
            [9, 8, 7, 6, 5, 4, 3, 2, 1]
        );
    }

#[no_mangle]
    #[test]
    fn zst_pain() {
        let mut rose = Rose::new();

        struct Empty;
        rose.push(Empty);
        rose.push(Empty);

        assert_eq!(rose.iter().count(), 2);
        assert_eq!(rose.iter_mut().count(), 2);
        assert_eq!(rose.into_iter().count(), 2);
    }
}
