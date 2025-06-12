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
impl<T: fmt::Debug, F> fmt::Debug for Lazy<T, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Lazy").field("cell", &self.cell).field("init", &"..").finish()
    }
}
