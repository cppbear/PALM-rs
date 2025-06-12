use core::{
    cell::{Cell, UnsafeCell},
    fmt, mem, ops::{Deref, DerefMut},
    panic::{RefUnwindSafe, UnwindSafe},
};
pub struct OnceCell<T> {
    inner: UnsafeCell<Option<T>>,
}
impl<T> OnceCell<T> {
    pub const fn new() -> OnceCell<T> {}
    pub const fn with_value(value: T) -> OnceCell<T> {}
    #[inline]
    pub fn get(&self) -> Option<&T> {}
    #[inline]
    pub fn get_mut(&mut self) -> Option<&mut T> {}
    pub fn set(&self, value: T) -> Result<(), T> {}
    pub fn try_insert(&self, value: T) -> Result<&T, (&T, T)> {}
    pub fn get_or_init<F>(&self, f: F) -> &T
    where
        F: FnOnce() -> T,
    {}
    pub fn get_or_try_init<F, E>(&self, f: F) -> Result<&T, E>
    where
        F: FnOnce() -> Result<T, E>,
    {}
    pub fn take(&mut self) -> Option<T> {
        mem::take(self).into_inner()
    }
    pub fn into_inner(self) -> Option<T> {
        self.inner.into_inner()
    }
}
