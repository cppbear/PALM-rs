use crate::BufMut;

use std::{cmp, io};

/// A `BufMut` adapter which implements `io::Write` for the inner value.
///
/// This struct is generally created by calling `writer()` on `BufMut`. See
/// documentation of [`writer()`](BufMut::writer) for more
/// details.
#[derive(Debug)]
pub struct Writer<B> {
    buf: B,
}

pub fn new<B>(buf: B) -> Writer<B> {
    Writer { buf }
}

impl<B: BufMut> Writer<B> {
    /// Gets a reference to the underlying `BufMut`.
    ///
    /// It is inadvisable to directly write to the underlying `BufMut`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytes::BufMut;
    ///
    /// let buf = Vec::with_capacity(1024).writer();
    ///
    /// assert_eq!(1024, buf.get_ref().capacity());
    /// ```
    pub fn get_ref(&self) -> &B {
        &self.buf
    }

    /// Gets a mutable reference to the underlying `BufMut`.
    ///
    /// It is inadvisable to directly write to the underlying `BufMut`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytes::BufMut;
    ///
    /// let mut buf = vec![].writer();
    ///
    /// buf.get_mut().reserve(1024);
    ///
    /// assert_eq!(1024, buf.get_ref().capacity());
    /// ```
    pub fn get_mut(&mut self) -> &mut B {
        &mut self.buf
    }

    /// Consumes this `Writer`, returning the underlying value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytes::BufMut;
    /// use std::io;
    ///
    /// let mut buf = vec![].writer();
    /// let mut src = &b"hello world"[..];
    ///
    /// io::copy(&mut src, &mut buf).unwrap();
    ///
    /// let buf = buf.into_inner();
    /// assert_eq!(*buf, b"hello world"[..]);
    /// ```
    pub fn into_inner(self) -> B {
        self.buf
    }
}

impl<B: BufMut + Sized> io::Write for Writer<B> {
    fn write(&mut self, src: &[u8]) -> io::Result<usize> {
        let n = cmp::min(self.buf.remaining_mut(), src.len());

        self.buf.put_slice(&src[..n]);
        Ok(n)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::iter::IntoIterator;
	use std::cmp::Ord;
	use std::iter::FromIterator;
	use std::default::Default;
	use std::cmp::PartialEq;
	use std::convert::From;
	use std::ops::Deref;
	use std::borrow::Borrow;
// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_22() {
//     rusty_monitor::set_test_id(22);
//     let mut usize_0: usize = 9714usize;
//     let mut isize_0: isize = 11337isize;
//     let mut take_0: crate::buf::take::Take<isize> = crate::buf::take::new(isize_0, usize_0);
//     let mut take_0_ref_0: &crate::buf::take::Take<isize> = &mut take_0;
//     let mut bytes_0: crate::bytes::Bytes = crate::bytes::Bytes::new();
//     let mut bytes_0_ref_0: &crate::bytes::Bytes = &mut bytes_0;
//     let mut str_0: &str = "AaZJ";
//     let mut str_0_ref_0: &str = &mut str_0;
//     let mut bytesmut_0: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from(str_0_ref_0);
//     let mut bytesmut_0_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_0;
//     let mut usize_1: usize = 5397usize;
//     let mut usize_2: usize = 8528usize;
//     let mut trygeterror_0: crate::TryGetError = crate::TryGetError {requested: usize_2, available: usize_1};
//     let mut result_0: std::result::Result<u16, crate::TryGetError> = std::result::Result::Err(trygeterror_0);
//     let mut writer_0: crate::buf::writer::Writer<std::result::Result<u16, crate::TryGetError>> = crate::buf::writer::Writer {buf: result_0};
//     let mut writer_0_ref_0: &crate::buf::writer::Writer<std::result::Result<u16, crate::TryGetError>> = &mut writer_0;
//     let mut usize_3: usize = 7063usize;
//     let mut bytesmut_1: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::zeroed(usize_3);
//     let mut bytesmut_1_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_1;
//     let mut usize_4: usize = 3318usize;
//     let mut usize_5: usize = 5410usize;
//     let mut bytesmut_2: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::with_capacity(usize_5);
//     let mut bytesmut_2_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_2;
//     let mut box_0: std::boxed::Box<[u8]> = std::boxed::Box::new();
//     let mut bytes_1: crate::bytes::Bytes = crate::bytes::Bytes::from(box_0);
//     let mut bytes_1_ref_0: &crate::bytes::Bytes = &mut bytes_1;
//     let mut iter_0: std::slice::Iter<u8> = crate::bytes::Bytes::into_iter(bytes_1_ref_0);
//     crate::bytes_mut::BytesMut::reserve(bytesmut_2_ref_0, usize_4);
//     let mut u8_slice_0: &[u8] = crate::bytes_mut::BytesMut::borrow(bytesmut_1_ref_0);
//     crate::bytes_mut::BytesMut::clear(bytesmut_0_ref_0);
//     let mut isize_1: &isize = crate::buf::take::Take::get_ref(take_0_ref_0);
//     panic!("From RustyUnit with love");
// }

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_27() {
    rusty_monitor::set_test_id(27);
    let mut usize_0: usize = 1943usize;
    let mut str_0: &str = "s8q9";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytesmut_0: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from(str_0_ref_0);
    let mut bytesmut_0_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_0;
    let mut f64_0: f64 = 7696.108741f64;
    let mut result_0: std::result::Result<f64, crate::TryGetError> = std::result::Result::Ok(f64_0);
    let mut writer_0: crate::buf::writer::Writer<std::result::Result<f64, crate::TryGetError>> = crate::buf::writer::Writer {buf: result_0};
    let mut writer_0_ref_0: &crate::buf::writer::Writer<std::result::Result<f64, crate::TryGetError>> = &mut writer_0;
    let mut str_1: &str = "y";
    let mut string_0: std::string::String = std::string::String::from(str_1);
    let mut bytes_0: crate::bytes::Bytes = crate::bytes::Bytes::from(string_0);
    let mut bytes_0_ref_0: &crate::bytes::Bytes = &mut bytes_0;
    let mut bytesmut_1: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::default();
    let mut bytesmut_1_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_1;
    let mut usize_1: usize = 1454usize;
    let mut isize_0: isize = -9964isize;
    let mut usize_2: usize = 1512usize;
    let mut bytesmut_2: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::with_capacity(usize_2);
    let mut bytesmut_2_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_2;
    let mut vec_0: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut bytesmut_3: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from_iter::<std::vec::Vec<u8>>(vec_0);
    let mut bytesmut_3_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_3;
    let mut vec_1: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut bytesmut_4: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from_iter::<std::vec::Vec<u8>>(vec_1);
    let mut bytesmut_4_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_4;
    let mut u8_slice_0: &[u8] = crate::bytes_mut::BytesMut::deref(bytesmut_4_ref_0);
    let mut ordering_0: std::cmp::Ordering = crate::bytes_mut::BytesMut::cmp(bytesmut_3_ref_0, bytesmut_2_ref_0);
    let mut take_0: crate::buf::take::Take<isize> = crate::buf::take::new(isize_0, usize_1);
    let mut bool_0: bool = crate::bytes_mut::BytesMut::eq(bytesmut_1_ref_0, bytes_0_ref_0);
    let mut take_0_ref_0: &mut crate::buf::take::Take<isize> = &mut take_0;
    let mut isize_1: &mut isize = crate::buf::take::Take::get_mut(take_0_ref_0);
    crate::bytes_mut::BytesMut::truncate(bytesmut_0_ref_0, usize_0);
    panic!("From RustyUnit with love");
}
}