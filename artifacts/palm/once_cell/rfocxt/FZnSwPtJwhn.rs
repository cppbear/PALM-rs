use core::{
    cell::{Cell, UnsafeCell},
    fmt, mem, ops::{Deref, DerefMut},
    panic::{RefUnwindSafe, UnwindSafe},
};
pub struct Lazy<T, F = fn() -> T> {
    cell: OnceCell<T>,
    init: Cell<Option<F>>,
}
pub struct OnceCell<T> {
    inner: UnsafeCell<Option<T>>,
}
pub struct OnceCell<T>(Imp<T>);
#[derive(Debug)]
pub(crate) struct OnceCell<T> {
    queue: AtomicPtr<Waiter>,
    value: UnsafeCell<Option<T>>,
}
impl<T, F: FnOnce() -> T> Lazy<T, F> {
    pub fn force(this: &Lazy<T, F>) -> &T {}
    pub fn force_mut(this: &mut Lazy<T, F>) -> &mut T {}
    pub fn get(this: &Lazy<T, F>) -> Option<&T> {
        this.cell.get()
    }
    pub fn get_mut(this: &mut Lazy<T, F>) -> Option<&mut T> {}
}
impl<T> OnceCell<T> {
    pub const fn new() -> OnceCell<T> {}
    pub const fn with_value(value: T) -> OnceCell<T> {}
    #[inline]
    pub fn get(&self) -> Option<&T> {
        unsafe { &*self.inner.get() }.as_ref()
    }
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
    pub fn take(&mut self) -> Option<T> {}
    pub fn into_inner(self) -> Option<T> {}
}
