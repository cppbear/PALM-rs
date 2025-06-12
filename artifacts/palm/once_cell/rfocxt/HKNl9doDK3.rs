use core::{
    cell::Cell, fmt, mem, ops::{Deref, DerefMut},
    panic::RefUnwindSafe,
};
use super::imp::OnceCell as Imp;
pub struct Lazy<T, F = fn() -> T> {
    cell: OnceCell<T>,
    init: Cell<Option<F>>,
}
#[derive(Debug)]
pub(crate) struct OnceCell<T> {
    queue: AtomicPtr<Waiter>,
    value: UnsafeCell<Option<T>>,
}
pub struct OnceCell<T>(Imp<T>);
pub struct OnceCell<T> {
    inner: UnsafeCell<Option<T>>,
}
impl<T, F: FnOnce() -> T> Lazy<T, F> {
    pub fn force(this: &Lazy<T, F>) -> &T {
        this.cell
            .get_or_init(|| match this.init.take() {
                Some(f) => f(),
                None => panic!("Lazy instance has previously been poisoned"),
            })
    }
    pub fn force_mut(this: &mut Lazy<T, F>) -> &mut T {}
    pub fn get(this: &Lazy<T, F>) -> Option<&T> {}
    pub fn get_mut(this: &mut Lazy<T, F>) -> Option<&mut T> {}
}
