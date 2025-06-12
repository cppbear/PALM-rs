use crate::Buf;
use core::cmp;
#[cfg(feature = "std")]
use std::io::IoSlice;
#[derive(Debug)]
pub struct Take<T> {
    inner: T,
    limit: usize,
}
impl<T> Take<T> {
    pub fn into_inner(self) -> T {}
    pub fn get_ref(&self) -> &T {}
    pub fn get_mut(&mut self) -> &mut T {}
    pub fn limit(&self) -> usize {}
    pub fn set_limit(&mut self, lim: usize) {
        self.limit = lim;
    }
}
