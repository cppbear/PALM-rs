pub use crate::buf::{Buf, BufMut};
pub use crate::bytes::Bytes;
pub use crate::bytes_mut::BytesMut;
#[inline(always)]
#[cfg(feature = "std")]
fn min_u64_usize(a: u64, b: usize) -> usize {
    use core::convert::TryFrom;
    match usize::try_from(a) {
        Ok(a) => usize::min(a, b),
        Err(_) => b,
    }
}
