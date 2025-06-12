use crate::lib::fmt::{self, Write};
use crate::lib::str;
pub(super) struct Buf<'a> {
    bytes: &'a mut [u8],
    offset: usize,
}
impl<'a> Write for Buf<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if self.offset + s.len() > self.bytes.len() {
            Err(fmt::Error)
        } else {
            self.bytes[self.offset..self.offset + s.len()].copy_from_slice(s.as_bytes());
            self.offset += s.len();
            Ok(())
        }
    }
}
