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
#[derive(Debug)]
pub(crate) struct OnceCell<T> {
    queue: AtomicPtr<Waiter>,
    value: UnsafeCell<Option<T>>,
}
pub struct OnceCell<T>(Imp<T>);
impl<T, F> Lazy<T, F> {
    pub const fn new(init: F) -> Lazy<T, F> {
        Lazy {
            cell: OnceCell::new(),
            init: Cell::new(Some(init)),
        }
    }
    pub fn into_value(this: Lazy<T, F>) -> Result<T, F> {}
}
impl<T> OnceCell<T> {
    pub const fn new() -> OnceCell<T> {
        OnceCell {
            inner: UnsafeCell::new(None),
        }
    }
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
    pub fn take(&mut self) -> Option<T> {}
    pub fn into_inner(self) -> Option<T> {}
}
