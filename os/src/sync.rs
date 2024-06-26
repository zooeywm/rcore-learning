//! Uniprocessor interior mutability primitives

use core::cell::{RefCell, RefMut};

/// Wrap a static data structure inside it so that we are able to access it without any `unsafe`.
///
/// We should only use it in Uniprocessor.
///
/// In order to get mutable reference of inner data, call [`UpSafeCell::exclusive_access()`]
pub struct UPSafeCell<T> {
    inner: RefCell<T>,
}

unsafe impl<T> Sync for UPSafeCell<T> {}

impl<T> UPSafeCell<T> {
    /// User is responsible to guarantee that inner struct is only used in Uniprocessor.
    pub unsafe fn new(value: T) -> Self {
        Self {
            inner: RefCell::new(value),
        }
    }

    /// Exclusive access inner data in [`UpSafeCell`]. Panic if the data has been borrowed.
    pub fn exclusive_access(&self) -> RefMut<'_, T> {
        self.inner.borrow_mut()
    }
}
