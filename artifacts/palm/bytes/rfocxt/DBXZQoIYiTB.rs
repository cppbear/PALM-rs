pub use crate::buf::{Buf, BufMut};
pub use crate::bytes::Bytes;
pub use crate::bytes_mut::BytesMut;
#[inline]
fn offset_from(dst: *const u8, original: *const u8) -> usize {
    dst as usize - original as usize
}
