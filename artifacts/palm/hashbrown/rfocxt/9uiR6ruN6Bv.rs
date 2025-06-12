use core::{
    mem::ManuallyDrop, ops::{Deref, DerefMut},
    ptr,
};
pub struct ScopeGuard<T, F>
where
    F: FnMut(&mut T),
{
    dropfn: F,
    value: T,
}
impl<T, F> ScopeGuard<T, F>
where
    F: FnMut(&mut T),
{
    #[inline]
    pub fn into_inner(guard: Self) -> T {
        let guard = ManuallyDrop::new(guard);
        unsafe {
            let value = ptr::read(&guard.value);
            let _ = ptr::read(&guard.dropfn);
            value
        }
    }
}
