pub use crate::buf::{Buf, BufMut};
pub use crate::bytes::Bytes;
pub use crate::bytes_mut::BytesMut;
#[cold]
fn panic_does_not_fit(size: usize, nbytes: usize) -> ! {
    panic!(
        "size too large: the integer type can fit {} bytes, but nbytes is {}", size,
        nbytes
    );
}
