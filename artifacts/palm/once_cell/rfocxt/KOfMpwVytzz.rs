use core::{
    cell::Cell, fmt, mem, ops::{Deref, DerefMut},
    panic::RefUnwindSafe,
};
use super::imp::OnceCell as Imp;
pub struct OnceCell<T>(Imp<T>);
impl<T> OnceCell<T> {
    pub const fn new() -> OnceCell<T> {}
    pub const fn with_value(value: T) -> OnceCell<T> {}
    pub fn get(&self) -> Option<&T> {}
    #[cfg(feature = "std")]
    pub fn wait(&self) -> &T {}
    #[inline]
    pub fn get_mut(&mut self) -> Option<&mut T> {}
    #[inline]
    pub unsafe fn get_unchecked(&self) -> &T {}
    pub fn set(&self, value: T) -> Result<(), T> {}
    pub fn try_insert(&self, value: T) -> Result<&T, (&T, T)> {}
    pub fn get_or_init<F>(&self, f: F) -> &T
    where
        F: FnOnce() -> T,
    {
        enum Void {}
        match self.get_or_try_init(|| Ok::<T, Void>(f())) {
            Ok(val) => val,
            Err(void) => match void {}
        }
    }
    pub fn get_or_try_init<F, E>(&self, f: F) -> Result<&T, E>
    where
        F: FnOnce() -> Result<T, E>,
    {}
    pub fn take(&mut self) -> Option<T> {}
    #[inline]
    pub fn into_inner(self) -> Option<T> {}
}
