use core::{
    cell::Cell, fmt, mem, ops::{Deref, DerefMut},
    panic::RefUnwindSafe,
};
use super::imp::OnceCell as Imp;
pub struct Lazy<T, F = fn() -> T> {
    cell: OnceCell<T>,
    init: Cell<Option<F>>,
}
pub struct OnceCell<T>(Imp<T>);
pub struct OnceCell<T> {
    inner: UnsafeCell<Option<T>>,
}
#[derive(Debug)]
pub(crate) struct OnceCell<T> {
    queue: AtomicPtr<Waiter>,
    value: UnsafeCell<Option<T>>,
}
impl<T, F: FnOnce() -> T> DerefMut for Lazy<T, F> {
    fn deref_mut(&mut self) -> &mut T {
        Lazy::force_mut(self)
    }
}
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
