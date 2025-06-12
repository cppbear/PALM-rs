use crate::Buf;
use core::cmp;
#[cfg(feature = "std")]
use std::io::IoSlice;
#[derive(Debug)]
pub struct Take<T> {
    inner: T,
    limit: usize,
}
pub fn new<T>(inner: T, limit: usize) -> Take<T> {
    Take { inner, limit }
}
