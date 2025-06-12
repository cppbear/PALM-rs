use core::{
    cell::{Cell, UnsafeCell},
    fmt, mem, ops::{Deref, DerefMut},
    panic::{RefUnwindSafe, UnwindSafe},
};
pub struct Lazy<T, F = fn() -> T> {
    cell: OnceCell<T>,
    init: Cell<Option<F>>,
}
#[derive(Debug)]
pub(crate) struct OnceCell<T> {
    queue: AtomicPtr<Waiter>,
    value: UnsafeCell<Option<T>>,
}
pub struct OnceCell<T> {
    inner: UnsafeCell<Option<T>>,
}
pub struct OnceCell<T>(Imp<T>);
impl<T, F: FnOnce() -> T> Deref for Lazy<T, F> {
    type Target = T;
    fn deref(&self) -> &T {
        Lazy::force(self)
    }
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
