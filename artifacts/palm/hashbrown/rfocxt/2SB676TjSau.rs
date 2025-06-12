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
impl<T, F> DerefMut for ScopeGuard<T, F>
where
    F: FnMut(&mut T),
{
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        &mut self.value
    }
}
