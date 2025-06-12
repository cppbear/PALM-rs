// #![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]
// #![doc(test(
//     no_crate_inject,
//     attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
// ))]
// #![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! Provides abstractions for working with bytes.
//!
//! The `bytes` crate provides an efficient byte buffer structure
//! ([`Bytes`]) and traits for working with buffer
//! implementations ([`Buf`], [`BufMut`]).
//!
//! # `Bytes`
//!
//! `Bytes` is an efficient container for storing and operating on contiguous
//! slices of memory. It is intended for use primarily in networking code, but
//! could have applications elsewhere as well.
//!
//! `Bytes` values facilitate zero-copy network programming by allowing multiple
//! `Bytes` objects to point to the same underlying memory. This is managed by
//! using a reference count to track when the memory is no longer needed and can
//! be freed.
//!
//! A `Bytes` handle can be created directly from an existing byte store (such as `&[u8]`
//! or `Vec<u8>`), but usually a `BytesMut` is used first and written to. For
//! example:
//!
//! ```rust
//! use bytes::{BytesMut, BufMut};
//!
//! let mut buf = BytesMut::with_capacity(1024);
//! buf.put(&b"hello world"[..]);
//! buf.put_u16(1234);
//!
//! let a = buf.split();
//! assert_eq!(a, b"hello world\x04\xD2"[..]);
//!
//! buf.put(&b"goodbye world"[..]);
//!
//! let b = buf.split();
//! assert_eq!(b, b"goodbye world"[..]);
//!
//! assert_eq!(buf.capacity(), 998);
//! ```
//!
//! In the above example, only a single buffer of 1024 is allocated. The handles
//! `a` and `b` will share the underlying buffer and maintain indices tracking
//! the view into the buffer represented by the handle.
//!
//! See the [struct docs](`Bytes`) for more details.
//!
//! # `Buf`, `BufMut`
//!
//! These two traits provide read and write access to buffers. The underlying
//! storage may or may not be in contiguous memory. For example, `Bytes` is a
//! buffer that guarantees contiguous memory, but a [rope] stores the bytes in
//! disjoint chunks. `Buf` and `BufMut` maintain cursors tracking the current
//! position in the underlying byte storage. When bytes are read or written, the
//! cursor is advanced.
//!
//! [rope]: https://en.wikipedia.org/wiki/Rope_(data_structure)
//!
//! ## Relation with `Read` and `Write`
//!
//! At first glance, it may seem that `Buf` and `BufMut` overlap in
//! functionality with [`std::io::Read`] and [`std::io::Write`]. However, they
//! serve different purposes. A buffer is the value that is provided as an
//! argument to `Read::read` and `Write::write`. `Read` and `Write` may then
//! perform a syscall, which has the potential of failing. Operations on `Buf`
//! and `BufMut` are infallible.

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod buf;
pub use crate::buf::{Buf, BufMut};

mod bytes;
mod bytes_mut;
mod fmt;
mod loom;
pub use crate::bytes::Bytes;
pub use crate::bytes_mut::BytesMut;

// Optional Serde support
#[cfg(feature = "serde")]
mod serde;

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

#[inline(always)]
#[cfg(feature = "std")]
fn saturating_sub_usize_u64(a: usize, b: u64) -> usize {
    use core::convert::TryFrom;
    match usize::try_from(b) {
        Ok(b) => a.saturating_sub(b),
        Err(_) => 0,
    }
}

#[inline(always)]
#[cfg(feature = "std")]
fn min_u64_usize(a: u64, b: usize) -> usize {
    use core::convert::TryFrom;
    match usize::try_from(a) {
        Ok(a) => usize::min(a, b),
        Err(_) => b,
    }
}

/// Error type for the `try_get_` methods of [`Buf`].
/// Indicates that there were not enough remaining
/// bytes in the buffer while attempting
/// to get a value from a [`Buf`] with one
/// of the `try_get_` methods.
#[derive(Debug, PartialEq, Eq)]
pub struct TryGetError {
    /// The number of bytes necessary to get the value
    pub requested: usize,

    /// The number of bytes available in the buffer
    pub available: usize,
}

impl core::fmt::Display for TryGetError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(
            f,
            "Not enough bytes remaining in buffer to read value (requested {} but only {} available)",
            self.requested,
            self.available
        )
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TryGetError {}

#[cfg(feature = "std")]
impl From<TryGetError> for std::io::Error {
    fn from(error: TryGetError) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, error)
    }
}

/// Panic with a nice error message.
#[cold]
fn panic_advance(error_info: &TryGetError) -> ! {
    panic!(
        "advance out of bounds: the len is {} but advancing by {}",
        error_info.available, error_info.requested
    );
}

#[cold]
fn panic_does_not_fit(size: usize, nbytes: usize) -> ! {
    panic!(
        "size too large: the integer type can fit {} bytes, but nbytes is {}",
        size, nbytes
    );
}

/// Precondition: dst >= original
///
/// The following line is equivalent to:
///
/// ```rust,ignore
/// self.ptr.as_ptr().offset_from(ptr) as usize;
/// ```
///
/// But due to min rust is 1.39 and it is only stabilized
/// in 1.47, we cannot use it.
#[inline]
fn offset_from(dst: *const u8, original: *const u8) -> usize {
    dst as usize - original as usize
}

pub use ntest::timeout;
pub mod rusty_monitor;

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::iter::IntoIterator;
	use std::iter::FromIterator;
	use std::default::Default;
	use std::clone::Clone;
	use std::cmp::PartialEq;
	use std::convert::AsMut;
	use buf::buf_impl::Buf;
	use std::cmp::PartialOrd;
	use std::convert::From;
	use std::ops::Deref;
// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_3() {
//     rusty_monitor::set_test_id(3);
//     let mut usize_0: usize = 7780usize;
//     let mut usize_1: usize = 7118usize;
//     let mut usize_2: usize = 5349usize;
//     let mut isize_0: isize = -11696isize;
//     let mut take_0: crate::buf::take::Take<isize> = crate::buf::take::new(isize_0, usize_2);
//     let mut bytes_0: crate::bytes::Bytes = crate::bytes::Bytes::default();
//     let mut bytes_0_ref_0: &crate::bytes::Bytes = &mut bytes_0;
//     let mut usize_3: usize = 4622usize;
//     let mut bytesmut_0: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::new();
//     let mut bytes_1: crate::bytes::Bytes = crate::bytes::Bytes::from(bytesmut_0);
//     let mut bytes_1_ref_0: &crate::bytes::Bytes = &mut bytes_1;
//     let mut vec_0: std::vec::Vec<u8> = std::vec::Vec::new();
//     let mut bytesmut_1: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from_iter::<std::vec::Vec<u8>>(vec_0);
//     let mut bytesmut_1_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_1;
//     let mut usize_4: usize = 6929usize;
//     let mut bytesmut_2: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::zeroed(usize_4);
//     let mut bytesmut_2_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_2;
//     let mut usize_5: usize = 473usize;
//     let mut bytesmut_3: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::new();
//     let mut bytesmut_3_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_3;
//     let mut box_0: std::boxed::Box<[u8]> = std::boxed::Box::new();
//     let mut bytes_2: crate::bytes::Bytes = crate::bytes::Bytes::from(box_0);
//     let mut bytes_2_ref_0: &crate::bytes::Bytes = &mut bytes_2;
//     let mut bytesmut_4: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::split_off(bytesmut_3_ref_0, usize_5);
//     let mut option_0: std::option::Option<std::cmp::Ordering> = crate::bytes::Bytes::partial_cmp(bytes_1_ref_0, bytes_0_ref_0);
//     let mut isize_1: isize = crate::buf::take::Take::into_inner(take_0);
//     let mut bytesmut_4_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_4;
//     let mut ordering_0: std::cmp::Ordering = std::option::Option::unwrap(option_0);
//     crate::panic_does_not_fit(usize_1, usize_0);
//     let mut iter_0: std::slice::Iter<u8> = crate::bytes_mut::BytesMut::into_iter(bytesmut_1_ref_0);
//     panic!("From RustyUnit with love");
// }

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4() {
    rusty_monitor::set_test_id(4);
    let mut usize_0: usize = 5854usize;
    let mut usize_1: usize = 9403usize;
    let mut isize_0: isize = -1291isize;
    let mut take_0: crate::buf::take::Take<isize> = crate::buf::take::new(isize_0, usize_1);
    let mut take_0_ref_0: &mut crate::buf::take::Take<isize> = &mut take_0;
    let mut str_0: &str = "fL4QQwcrJA";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut usize_2: usize = 2841usize;
    let mut usize_3: usize = 1556usize;
    let mut trygeterror_0: crate::TryGetError = crate::TryGetError {requested: usize_3, available: usize_2};
    let mut trygeterror_0_ref_0: &crate::TryGetError = &mut trygeterror_0;
    let mut str_1: &str = "mI";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut usize_4: usize = 8535usize;
    let mut usize_5: usize = 7627usize;
    let mut isize_1: isize = 7498isize;
    let mut take_1: crate::buf::take::Take<isize> = crate::buf::take::new(isize_1, usize_5);
    let mut take_1_ref_0: &mut crate::buf::take::Take<isize> = &mut take_1;
    let mut bytes_0: crate::bytes::Bytes = crate::bytes::Bytes::new();
    let mut bytes_0_ref_0: &crate::bytes::Bytes = &mut bytes_0;
    let mut vec_0: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut bytes_1: crate::bytes::Bytes = crate::bytes::Bytes::from(vec_0);
    let mut bytes_1_ref_0: &crate::bytes::Bytes = &mut bytes_1;
    let mut u8_slice_0: &[u8] = crate::bytes::Bytes::deref(bytes_1_ref_0);
    let mut bytes_2: crate::bytes::Bytes = crate::bytes::Bytes::clone(bytes_0_ref_0);
    crate::buf::take::Take::set_limit(take_1_ref_0, usize_4);
    let mut bytes_2_ref_0: &crate::bytes::Bytes = &mut bytes_2;
    let mut option_0: std::option::Option<std::cmp::Ordering> = crate::bytes::Bytes::partial_cmp(bytes_2_ref_0, str_1_ref_0);
    crate::panic_advance(trygeterror_0_ref_0);
    let mut ordering_0: std::cmp::Ordering = std::option::Option::unwrap(option_0);
    let mut bytesmut_0: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from(str_0_ref_0);
    crate::buf::take::Take::set_limit(take_0_ref_0, usize_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7() {
    rusty_monitor::set_test_id(7);
    let mut f32_0: f32 = 5076.305094f32;
    let mut result_0: std::result::Result<f32, crate::TryGetError> = std::result::Result::Ok(f32_0);
    let mut writer_0: crate::buf::writer::Writer<std::result::Result<f32, crate::TryGetError>> = crate::buf::writer::new(result_0);
    let mut writer_0_ref_0: &crate::buf::writer::Writer<std::result::Result<f32, crate::TryGetError>> = &mut writer_0;
    let mut usize_0: usize = 1059usize;
    let mut bytes_0: crate::bytes::Bytes = crate::bytes::Bytes::new();
    let mut bytesmut_0: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from(bytes_0);
    let mut bytesmut_0_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_0;
    let mut usize_1: usize = 8370usize;
    let mut bytesmut_1: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::default();
    let mut bytesmut_1_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_1;
    let mut bytes_1: crate::bytes::Bytes = crate::bytes_mut::BytesMut::copy_to_bytes(bytesmut_1_ref_0, usize_1);
    let mut bytes_1_ref_0: &crate::bytes::Bytes = &mut bytes_1;
    let mut bytesmut_2: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::default();
    let mut bytesmut_2_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_2;
    let mut usize_2: usize = 2263usize;
    let mut bytesmut_3: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::default();
    let mut bytesmut_3_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_3;
    let mut usize_3: usize = 1569usize;
    let mut usize_4: usize = 1455usize;
    let mut bytesmut_4: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::default();
    let mut bytesmut_4_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_4;
    let mut vec_0: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut bytesmut_5: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from_iter::<std::vec::Vec<u8>>(vec_0);
    let mut bytesmut_5_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_5;
    let mut usize_5: usize = crate::bytes_mut::BytesMut::len(bytesmut_5_ref_0);
    crate::panic_does_not_fit(usize_4, usize_3);
    let mut bytesmut_6: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::clone(bytesmut_3_ref_0);
    crate::bytes_mut::BytesMut::reserve(bytesmut_0_ref_0, usize_0);
    let mut bytesmut_6_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_6;
    panic!("From RustyUnit with love");
}

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_45() {
//     rusty_monitor::set_test_id(45);
//     let mut bytesmut_0: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::default();
//     let mut bytesmut_0_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_0;
//     let mut usize_0: usize = 7163usize;
//     let mut bytesmut_1: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::with_capacity(usize_0);
//     let mut bytesmut_1_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_1;
//     let mut usize_1: usize = 8549usize;
//     let mut usize_2: usize = 1326usize;
//     let mut usize_3: usize = 4130usize;
//     let mut trygeterror_0: crate::TryGetError = crate::TryGetError {requested: usize_3, available: usize_2};
//     let mut trygeterror_0_ref_0: &crate::TryGetError = &mut trygeterror_0;
//     let mut usize_4: usize = 8234usize;
//     let mut usize_5: usize = 2132usize;
//     let mut trygeterror_1: crate::TryGetError = crate::TryGetError {requested: usize_5, available: usize_4};
//     let mut trygeterror_1_ref_0: &crate::TryGetError = &mut trygeterror_1;
//     let mut str_0: &str = "PhRtd7zEI";
//     let mut str_0_ref_0: &str = &mut str_0;
//     let mut bytesmut_2: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::new();
//     let mut bytes_0: crate::bytes::Bytes = crate::bytes::Bytes::default();
//     let mut bytes_0_ref_0: &crate::bytes::Bytes = &mut bytes_0;
//     let mut u64_0: u64 = 7326u64;
//     let mut usize_6: usize = 8798usize;
//     let mut usize_7: usize = crate::saturating_sub_usize_u64(usize_6, u64_0);
//     let mut iter_0: std::slice::Iter<u8> = crate::bytes::Bytes::into_iter(bytes_0_ref_0);
//     let mut bytes_1: crate::bytes::Bytes = crate::bytes::Bytes::from(bytesmut_2);
//     let mut bytesmut_3: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from(str_0_ref_0);
//     let mut bytesmut_3_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_3;
//     let mut bytes_1_ref_0: &crate::bytes::Bytes = &mut bytes_1;
//     let mut u8_slice_0: &[u8] = crate::bytes::Bytes::chunk(bytes_1_ref_0);
//     let mut iter_1: std::slice::Iter<u8> = crate::bytes_mut::BytesMut::into_iter(bytesmut_3_ref_0);
//     let mut bool_0: bool = crate::TryGetError::eq(trygeterror_1_ref_0, trygeterror_0_ref_0);
//     let mut bool_1: bool = crate::bytes_mut::BytesMut::is_empty(bytesmut_1_ref_0);
//     let mut u8_slice_1: &[u8] = crate::bytes_mut::BytesMut::deref(bytesmut_0_ref_0);
//     panic!("From RustyUnit with love");
// }

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_49() {
    rusty_monitor::set_test_id(49);
    let mut usize_0: usize = 5271usize;
    let mut bytesmut_0: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::zeroed(usize_0);
    let mut bytesmut_0_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_0;
    let mut str_0: &str = "VZ5h7mcY54Gsgn";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytesmut_1: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from(str_0_ref_0);
    let mut bytes_0: crate::bytes::Bytes = crate::bytes_mut::BytesMut::freeze(bytesmut_1);
    let mut bytes_0_ref_0: &crate::bytes::Bytes = &mut bytes_0;
    let mut usize_1: usize = 7832usize;
    let mut bytesmut_2: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::with_capacity(usize_1);
    let mut bytesmut_2_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_2;
    let mut usize_2: usize = 8564usize;
    let mut bytesmut_3: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::new();
    let mut bytesmut_3_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_3;
    let mut u64_0: u64 = 4265u64;
    let mut usize_3: usize = 6637usize;
    let mut usize_4: usize = 4000usize;
    let mut bytesmut_4: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::zeroed(usize_4);
    let mut bytesmut_4_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_4;
    let mut usize_5: usize = 4160usize;
    let mut bytesmut_5: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::with_capacity(usize_5);
    let mut bytes_1: crate::bytes::Bytes = crate::bytes_mut::BytesMut::freeze(bytesmut_5);
    let mut bytes_1_ref_0: &crate::bytes::Bytes = &mut bytes_1;
    let mut u8_slice_0: &[u8] = crate::bytes::Bytes::deref(bytes_1_ref_0);
    let mut u8_slice_1: &mut [u8] = crate::bytes_mut::BytesMut::as_mut(bytesmut_4_ref_0);
    let mut usize_6: usize = crate::saturating_sub_usize_u64(usize_3, u64_0);
    let mut bytesmut_6: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::split_to(bytesmut_3_ref_0, usize_2);
    let mut bytesmut_6_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_6;
    let mut bool_0: bool = crate::bytes_mut::BytesMut::eq(bytesmut_6_ref_0, bytesmut_2_ref_0);
    let mut bool_1: bool = crate::bytes::Bytes::eq(bytes_0_ref_0, bytesmut_0_ref_0);
    panic!("From RustyUnit with love");
}
}