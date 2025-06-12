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
impl<T, F> Deref for ScopeGuard<T, F>
where
    F: FnMut(&mut T),
{
    type Target = T;
    #[inline]
    fn deref(&self) -> &T {
        &self.value
    }
}
