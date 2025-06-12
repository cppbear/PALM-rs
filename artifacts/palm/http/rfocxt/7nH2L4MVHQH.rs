use bytes::Bytes;
use std::{ops, str};
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(crate) struct ByteStr {
    bytes: Bytes,
}
impl ByteStr {
    #[inline]
    pub fn new() -> ByteStr {}
    #[inline]
    pub const fn from_static(val: &'static str) -> ByteStr {}
    #[inline]
    pub unsafe fn from_utf8_unchecked(bytes: Bytes) -> ByteStr {
        if cfg!(debug_assertions) {
            match str::from_utf8(&bytes) {
                Ok(_) => {}
                Err(err) => {
                    panic!(
                        "ByteStr::from_utf8_unchecked() with invalid bytes; error = {}, bytes = {:?}",
                        err, bytes
                    )
                }
            }
        }
        ByteStr { bytes }
    }
    pub(crate) fn from_utf8(bytes: Bytes) -> Result<ByteStr, std::str::Utf8Error> {}
}
