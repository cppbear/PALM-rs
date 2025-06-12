pub use crate::buf::{Buf, BufMut};
pub use crate::bytes::Bytes;
pub use crate::bytes_mut::BytesMut;
#[inline(never)]
#[cold]
fn abort() -> ! {
    #[cfg(feature = "std")]
    {
        std::process::abort();
    }
    #[cfg(not(feature = "std"))]
    {
        struct Abort;
        impl Drop for Abort {
            fn drop(&mut self) {
                panic!();
            }
        }
        let _a = Abort;
        panic!("abort");
    }
}
