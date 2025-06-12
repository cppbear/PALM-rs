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
impl<T, F: FnOnce() -> T> Lazy<T, F> {
    pub fn force(this: &Lazy<T, F>) -> &T {}
    pub fn force_mut(this: &mut Lazy<T, F>) -> &mut T {
        if this.cell.get_mut().is_none() {
            let value = match this.init.get_mut().take() {
                Some(f) => f(),
                None => panic!("Lazy instance has previously been poisoned"),
            };
            this.cell = OnceCell::with_value(value);
        }
        this.cell.get_mut().unwrap_or_else(|| unreachable!())
    }
    pub fn get(this: &Lazy<T, F>) -> Option<&T> {}
    pub fn get_mut(this: &mut Lazy<T, F>) -> Option<&mut T> {}
}
impl<T> OnceCell<T> {
    pub const fn new() -> OnceCell<T> {}
    pub const fn with_value(value: T) -> OnceCell<T> {
        OnceCell {
            inner: UnsafeCell::new(Some(value)),
        }
    }
    #[inline]
    pub fn get(&self) -> Option<&T> {}
    #[inline]
    pub fn get_mut(&mut self) -> Option<&mut T> {
        unsafe { &mut *self.inner.get() }.as_mut()
    }
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
