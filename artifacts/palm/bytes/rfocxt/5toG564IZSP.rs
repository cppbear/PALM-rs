use core::fmt::{Formatter, LowerHex, Result, UpperHex};
use super::BytesRef;
use crate::{Bytes, BytesMut};
fmt_impl!(LowerHex, Bytes);
fmt_impl!(LowerHex, BytesMut);
fmt_impl!(UpperHex, Bytes);
fmt_impl!(UpperHex, BytesMut);
struct BytesRef<'a>(&'a [u8]);
impl LowerHex for BytesRef<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for &b in self.0 {
            write!(f, "{:02x}", b)?;
        }
        Ok(())
    }
}
