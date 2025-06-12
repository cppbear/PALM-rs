use core::fmt::{Debug, Formatter, Result};
use super::BytesRef;
use crate::{Bytes, BytesMut};
fmt_impl!(Debug, Bytes);
fmt_impl!(Debug, BytesMut);
struct BytesRef<'a>(&'a [u8]);
impl Debug for BytesRef<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "b\"")?;
        for &b in self.0 {
            if b == b'\n' {
                write!(f, "\\n")?;
            } else if b == b'\r' {
                write!(f, "\\r")?;
            } else if b == b'\t' {
                write!(f, "\\t")?;
            } else if b == b'\\' || b == b'"' {
                write!(f, "\\{}", b as char)?;
            } else if b == b'\0' {
                write!(f, "\\0")?;
            } else if (0x20..0x7f).contains(&b) {
                write!(f, "{}", b as char)?;
            } else {
                write!(f, "\\x{:02x}", b)?;
            }
        }
        write!(f, "\"")?;
        Ok(())
    }
}
