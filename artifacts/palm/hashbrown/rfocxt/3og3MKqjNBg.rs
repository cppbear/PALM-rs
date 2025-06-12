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
#[inline]
pub fn guard<T, F>(value: T, dropfn: F) -> ScopeGuard<T, F>
where
    F: FnMut(&mut T),
{
    ScopeGuard { dropfn, value }
}
