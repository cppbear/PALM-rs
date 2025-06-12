use super::atomic::{AtomicPtr, Ordering};
use core::{marker::PhantomData, ptr};
use alloc::boxed::Box;
pub struct OnceBox<T> {
    inner: AtomicPtr<T>,
    ghost: PhantomData<Option<Box<T>>>,
}
impl<T> core::fmt::Debug for OnceBox<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "OnceBox({:?})", self.inner.load(Ordering::Relaxed))
    }
}
