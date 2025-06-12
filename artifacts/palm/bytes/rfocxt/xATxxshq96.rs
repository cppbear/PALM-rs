use crate::buf::UninitSlice;
use crate::BufMut;
use core::cmp;
#[derive(Debug)]
pub struct Limit<T> {
    inner: T,
    limit: usize,
}
pub(super) fn new<T>(inner: T, limit: usize) -> Limit<T> {
    Limit { inner, limit }
}
