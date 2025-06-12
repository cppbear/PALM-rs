use crate::lib::fmt::{self, Write};
use crate::lib::str;
pub(super) struct Buf<'a> {
    bytes: &'a mut [u8],
    offset: usize,
}
impl<'a> Buf<'a> {
    pub fn new(bytes: &'a mut [u8]) -> Self {
        Buf { bytes, offset: 0 }
    }
    pub fn as_str(&self) -> &str {
        let slice = &self.bytes[..self.offset];
        unsafe { str::from_utf8_unchecked(slice) }
    }
}
