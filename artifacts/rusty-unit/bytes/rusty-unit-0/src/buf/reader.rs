use crate::Buf;

use std::{cmp, io};

/// A `Buf` adapter which implements `io::Read` for the inner value.
///
/// This struct is generally created by calling `reader()` on `Buf`. See
/// documentation of [`reader()`](Buf::reader) for more
/// details.
#[derive(Debug)]
pub struct Reader<B> {
    buf: B,
}

pub fn new<B>(buf: B) -> Reader<B> {
    Reader { buf }
}

impl<B: Buf> Reader<B> {
    /// Gets a reference to the underlying `Buf`.
    ///
    /// It is inadvisable to directly read from the underlying `Buf`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytes::Buf;
    ///
    /// let buf = b"hello world".reader();
    ///
    /// assert_eq!(b"hello world", buf.get_ref());
    /// ```
    pub fn get_ref(&self) -> &B {
        &self.buf
    }

    /// Gets a mutable reference to the underlying `Buf`.
    ///
    /// It is inadvisable to directly read from the underlying `Buf`.
    pub fn get_mut(&mut self) -> &mut B {
        &mut self.buf
    }

    /// Consumes this `Reader`, returning the underlying value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytes::Buf;
    /// use std::io;
    ///
    /// let mut buf = b"hello world".reader();
    /// let mut dst = vec![];
    ///
    /// io::copy(&mut buf, &mut dst).unwrap();
    ///
    /// let buf = buf.into_inner();
    /// assert_eq!(0, buf.remaining());
    /// ```
    pub fn into_inner(self) -> B {
        self.buf
    }
}

impl<B: Buf + Sized> io::Read for Reader<B> {
    fn read(&mut self, dst: &mut [u8]) -> io::Result<usize> {
        let len = cmp::min(self.buf.remaining(), dst.len());

        Buf::copy_to_slice(&mut self.buf, &mut dst[0..len]);
        Ok(len)
    }
}

impl<B: Buf + Sized> io::BufRead for Reader<B> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        Ok(self.buf.chunk())
    }
    fn consume(&mut self, amt: usize) {
        self.buf.advance(amt)
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::iter::IntoIterator;
	use std::default::Default;
	use std::cmp::PartialEq;
	use std::clone::Clone;
	use buf::buf_impl::Buf;
	use std::convert::AsRef;
	use std::ops::Deref;
	use std::convert::From;
	use std::borrow::Borrow;
// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_30() {
//     rusty_monitor::set_test_id(30);
//     let mut bytesmut_0: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::default();
//     let mut bytesmut_0_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_0;
//     let mut usize_0: usize = 6504usize;
//     let mut usize_1: usize = 4755usize;
//     let mut bytesmut_1: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::default();
//     let mut bytesmut_1_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_1;
//     let mut bytes_0: crate::bytes::Bytes = crate::bytes_mut::BytesMut::copy_to_bytes(bytesmut_1_ref_0, usize_1);
//     let mut bytes_0_ref_0: &crate::bytes::Bytes = &mut bytes_0;
//     let mut bytes_1: crate::bytes::Bytes = crate::bytes::Bytes::default();
//     let mut bytes_1_ref_0: &crate::bytes::Bytes = &mut bytes_1;
//     let mut usize_2: usize = 7311usize;
//     let mut bytesmut_2: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::with_capacity(usize_2);
//     let mut bytesmut_2_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_2;
//     let mut usize_3: usize = 326usize;
//     let mut usize_4: usize = 8866usize;
//     let mut usize_5: usize = 7517usize;
//     let mut trygeterror_0: crate::TryGetError = crate::TryGetError {requested: usize_5, available: usize_4};
//     let mut result_0: std::result::Result<(), crate::TryGetError> = std::result::Result::Err(trygeterror_0);
//     let mut reader_0: crate::buf::reader::Reader<std::result::Result<(), crate::TryGetError>> = crate::buf::reader::Reader {buf: result_0};
//     let mut reader_0_ref_0: &crate::buf::reader::Reader<std::result::Result<(), crate::TryGetError>> = &mut reader_0;
//     let mut usize_6: usize = 6667usize;
//     let mut bytesmut_3: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::with_capacity(usize_6);
//     let mut bytesmut_3_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_3;
//     let mut iter_0: std::slice::Iter<u8> = crate::bytes_mut::BytesMut::into_iter(bytesmut_3_ref_0);
//     let mut bytesmut_4: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::default();
//     let mut bytesmut_4_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_4;
//     let mut u8_slice_0: &[u8] = crate::bytes_mut::BytesMut::deref(bytesmut_4_ref_0);
//     let mut u8_slice_1: &[u8] = crate::bytes_mut::BytesMut::borrow(bytesmut_2_ref_0);
//     let mut bool_0: bool = crate::bytes::Bytes::eq(bytes_1_ref_0, bytes_0_ref_0);
//     let mut bytesmut_5: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::clone(bytesmut_0_ref_0);
//     panic!("From RustyUnit with love");
// }

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_38() {
    rusty_monitor::set_test_id(38);
    let mut bytes_0: crate::bytes::Bytes = crate::bytes::Bytes::new();
    let mut bytesmut_0: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from(bytes_0);
    let mut bytesmut_0_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_0;
    let mut usize_0: usize = 753usize;
    let mut usize_1: usize = 6712usize;
    let mut bytesmut_1: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::zeroed(usize_1);
    let mut bytesmut_2: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::default();
    let mut bytesmut_2_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_2;
    let mut bytesmut_3: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::default();
    let mut bytesmut_3_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_3;
    let mut u8_0: u8 = 14u8;
    let mut usize_2: usize = 2167usize;
    let mut str_0: &str = "HDsjjuzTHs9xCfVeuNC";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytes_1: crate::bytes::Bytes = crate::bytes::Bytes::from(str_0_ref_0);
    let mut bytesmut_4: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from(bytes_1);
    let mut bytesmut_4_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_4;
    let mut usize_3: usize = 709usize;
    let mut str_1: &str = "c7PWur4UtKHtF0zBM";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytesmut_5: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from(str_1_ref_0);
    let mut bytesmut_5_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_5;
    let mut usize_4: usize = 6869usize;
    let mut usize_5: usize = 8310usize;
    let mut trygeterror_0: crate::TryGetError = crate::TryGetError {requested: usize_5, available: usize_4};
    let mut result_0: std::result::Result<u32, crate::TryGetError> = std::result::Result::Err(trygeterror_0);
    let mut reader_0: crate::buf::reader::Reader<std::result::Result<u32, crate::TryGetError>> = crate::buf::reader::Reader {buf: result_0};
    let mut reader_0_ref_0: &crate::buf::reader::Reader<std::result::Result<u32, crate::TryGetError>> = &mut reader_0;
    let mut bytes_2: crate::bytes::Bytes = crate::bytes_mut::BytesMut::copy_to_bytes(bytesmut_5_ref_0, usize_3);
    crate::bytes_mut::BytesMut::resize(bytesmut_4_ref_0, usize_2, u8_0);
    let mut bytes_2_ref_0: &crate::bytes::Bytes = &mut bytes_2;
    let mut u8_slice_0: &[u8] = crate::bytes::Bytes::as_ref(bytes_2_ref_0);
    crate::bytes_mut::BytesMut::unsplit(bytesmut_2_ref_0, bytesmut_1);
    panic!("From RustyUnit with love");
}
}