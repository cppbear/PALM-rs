pub use crate::buf::{Buf, BufMut};
pub use crate::bytes::Bytes;
pub use crate::bytes_mut::BytesMut;
#[inline(always)]
#[cfg(feature = "std")]
fn saturating_sub_usize_u64(a: usize, b: u64) -> usize {
    use core::convert::TryFrom;
    match usize::try_from(b) {
        Ok(b) => a.saturating_sub(b),
        Err(_) => 0,
    }
}
