use crate::buf::UninitSlice;
use crate::BufMut;
use core::cmp;
#[derive(Debug)]
pub struct Limit<T> {
    inner: T,
    limit: usize,
}
impl<T> Limit<T> {
    pub fn into_inner(self) -> T {}
    pub fn get_ref(&self) -> &T {}
    pub fn get_mut(&mut self) -> &mut T {}
    pub fn limit(&self) -> usize {
        self.limit
    }
    pub fn set_limit(&mut self, lim: usize) {}
}
