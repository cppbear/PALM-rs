#[cfg(not(feature = "portable-atomic"))]
use core::sync::atomic;
#[cfg(feature = "portable-atomic")]
use portable_atomic as atomic;
use atomic::{AtomicPtr, AtomicUsize, Ordering};
use core::cell::UnsafeCell;
use core::marker::PhantomData;
use core::num::NonZeroUsize;
use core::ptr;
#[cfg(feature = "alloc")]
pub use self::once_box::OnceBox;
pub struct OnceRef<'a, T> {
    inner: AtomicPtr<T>,
    ghost: PhantomData<UnsafeCell<&'a T>>,
}
impl<'a, T> OnceRef<'a, T> {
    pub const fn new() -> Self {
        Self {
            inner: AtomicPtr::new(ptr::null_mut()),
            ghost: PhantomData,
        }
    }
    pub fn get(&self) -> Option<&'a T> {}
    pub fn set(&self, value: &'a T) -> Result<(), ()> {}
    pub fn get_or_init<F>(&self, f: F) -> &'a T
    where
        F: FnOnce() -> &'a T,
    {
        enum Void {}
        match self.get_or_try_init(|| Ok::<&'a T, Void>(f())) {
            Ok(val) => val,
            Err(void) => match void {}
        }
    }
    pub fn get_or_try_init<F, E>(&self, f: F) -> Result<&'a T, E>
    where
        F: FnOnce() -> Result<&'a T, E>,
    {}
    #[cold]
    #[inline(never)]
    fn init<E>(&self, f: impl FnOnce() -> Result<&'a T, E>) -> Result<&'a T, E> {}
    #[inline(always)]
    fn compare_exchange(&self, value: &'a T) -> Result<(), *const T> {}
    fn _dummy() {}
}
