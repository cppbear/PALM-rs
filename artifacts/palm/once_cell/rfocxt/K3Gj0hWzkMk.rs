use super::atomic::{AtomicPtr, Ordering};
use core::{marker::PhantomData, ptr};
use alloc::boxed::Box;
pub struct OnceBox<T> {
    inner: AtomicPtr<T>,
    ghost: PhantomData<Option<Box<T>>>,
}
impl<T> OnceBox<T> {
    pub const fn new() -> Self {
        Self {
            inner: AtomicPtr::new(ptr::null_mut()),
            ghost: PhantomData,
        }
    }
    pub fn with_value(value: Box<T>) -> Self {
        Self {
            inner: AtomicPtr::new(Box::into_raw(value)),
            ghost: PhantomData,
        }
    }
    pub fn get(&self) -> Option<&T> {
        let ptr = self.inner.load(Ordering::Acquire);
        if ptr.is_null() {
            return None;
        }
        Some(unsafe { &*ptr })
    }
    pub fn set(&self, value: Box<T>) -> Result<(), Box<T>> {}
    pub fn get_or_init<F>(&self, f: F) -> &T
    where
        F: FnOnce() -> Box<T>,
    {}
    pub fn get_or_try_init<F, E>(&self, f: F) -> Result<&T, E>
    where
        F: FnOnce() -> Result<Box<T>, E>,
    {}
    #[cold]
    #[inline(never)]
    fn init<E>(&self, f: impl FnOnce() -> Result<Box<T>, E>) -> Result<&T, E> {}
}
