use bytes::Bytes;
use std::{ops, str};
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(crate) struct ByteStr {
    bytes: Bytes,
}
impl ByteStr {
    #[inline]
    pub fn new() -> ByteStr {
        ByteStr { bytes: Bytes::new() }
    }
    #[inline]
    pub const fn from_static(val: &'static str) -> ByteStr {}
    #[inline]
    pub unsafe fn from_utf8_unchecked(bytes: Bytes) -> ByteStr {}
    pub(crate) fn from_utf8(bytes: Bytes) -> Result<ByteStr, std::str::Utf8Error> {}
}
