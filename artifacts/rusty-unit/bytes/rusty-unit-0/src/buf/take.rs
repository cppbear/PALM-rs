use crate::Buf;

use core::cmp;

#[cfg(feature = "std")]
use std::io::IoSlice;

/// A `Buf` adapter which limits the bytes read from an underlying buffer.
///
/// This struct is generally created by calling `take()` on `Buf`. See
/// documentation of [`take()`](Buf::take) for more details.
#[derive(Debug)]
pub struct Take<T> {
    inner: T,
    limit: usize,
}

pub fn new<T>(inner: T, limit: usize) -> Take<T> {
    Take { inner, limit }
}

impl<T> Take<T> {
    /// Consumes this `Take`, returning the underlying value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytes::{Buf, BufMut};
    ///
    /// let mut buf = b"hello world".take(2);
    /// let mut dst = vec![];
    ///
    /// dst.put(&mut buf);
    /// assert_eq!(*dst, b"he"[..]);
    ///
    /// let mut buf = buf.into_inner();
    ///
    /// dst.clear();
    /// dst.put(&mut buf);
    /// assert_eq!(*dst, b"llo world"[..]);
    /// ```
    pub fn into_inner(self) -> T {
        self.inner
    }

    /// Gets a reference to the underlying `Buf`.
    ///
    /// It is inadvisable to directly read from the underlying `Buf`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytes::Buf;
    ///
    /// let buf = b"hello world".take(2);
    ///
    /// assert_eq!(11, buf.get_ref().remaining());
    /// ```
    pub fn get_ref(&self) -> &T {
        &self.inner
    }

    /// Gets a mutable reference to the underlying `Buf`.
    ///
    /// It is inadvisable to directly read from the underlying `Buf`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytes::{Buf, BufMut};
    ///
    /// let mut buf = b"hello world".take(2);
    /// let mut dst = vec![];
    ///
    /// buf.get_mut().advance(2);
    ///
    /// dst.put(&mut buf);
    /// assert_eq!(*dst, b"ll"[..]);
    /// ```
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    /// Returns the maximum number of bytes that can be read.
    ///
    /// # Note
    ///
    /// If the inner `Buf` has fewer bytes than indicated by this method then
    /// that is the actual number of available bytes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytes::Buf;
    ///
    /// let mut buf = b"hello world".take(2);
    ///
    /// assert_eq!(2, buf.limit());
    /// assert_eq!(b'h', buf.get_u8());
    /// assert_eq!(1, buf.limit());
    /// ```
    pub fn limit(&self) -> usize {
        self.limit
    }

    /// Sets the maximum number of bytes that can be read.
    ///
    /// # Note
    ///
    /// If the inner `Buf` has fewer bytes than `lim` then that is the actual
    /// number of available bytes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytes::{Buf, BufMut};
    ///
    /// let mut buf = b"hello world".take(2);
    /// let mut dst = vec![];
    ///
    /// dst.put(&mut buf);
    /// assert_eq!(*dst, b"he"[..]);
    ///
    /// dst.clear();
    ///
    /// buf.set_limit(3);
    /// dst.put(&mut buf);
    /// assert_eq!(*dst, b"llo"[..]);
    /// ```
    pub fn set_limit(&mut self, lim: usize) {
        self.limit = lim
    }
}

impl<T: Buf> Buf for Take<T> {
    fn remaining(&self) -> usize {
        cmp::min(self.inner.remaining(), self.limit)
    }

    fn chunk(&self) -> &[u8] {
        let bytes = self.inner.chunk();
        &bytes[..cmp::min(bytes.len(), self.limit)]
    }

    fn advance(&mut self, cnt: usize) {
        assert!(cnt <= self.limit);
        self.inner.advance(cnt);
        self.limit -= cnt;
    }

    fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes {
        assert!(len <= self.remaining(), "`len` greater than remaining");

        let r = self.inner.copy_to_bytes(len);
        self.limit -= len;
        r
    }

    #[cfg(feature = "std")]
    fn chunks_vectored<'a>(&'a self, dst: &mut [IoSlice<'a>]) -> usize {
        if self.limit == 0 {
            return 0;
        }

        const LEN: usize = 16;
        let mut slices: [IoSlice<'a>; LEN] = [
            IoSlice::new(&[]),
            IoSlice::new(&[]),
            IoSlice::new(&[]),
            IoSlice::new(&[]),
            IoSlice::new(&[]),
            IoSlice::new(&[]),
            IoSlice::new(&[]),
            IoSlice::new(&[]),
            IoSlice::new(&[]),
            IoSlice::new(&[]),
            IoSlice::new(&[]),
            IoSlice::new(&[]),
            IoSlice::new(&[]),
            IoSlice::new(&[]),
            IoSlice::new(&[]),
            IoSlice::new(&[]),
        ];

        let cnt = self
            .inner
            .chunks_vectored(&mut slices[..dst.len().min(LEN)]);
        let mut limit = self.limit;
        for (i, (dst, slice)) in dst[..cnt].iter_mut().zip(slices.iter()).enumerate() {
            if let Some(buf) = slice.get(..limit) {
                // SAFETY: We could do this safely with `IoSlice::advance` if we had a larger MSRV.
                let buf = unsafe { std::mem::transmute::<&[u8], &'a [u8]>(buf) };
                *dst = IoSlice::new(buf);
                return i + 1;
            } else {
                // SAFETY: We could do this safely with `IoSlice::advance` if we had a larger MSRV.
                let buf = unsafe { std::mem::transmute::<&[u8], &'a [u8]>(slice) };
                *dst = IoSlice::new(buf);
                limit -= slice.len();
            }
        }
        cnt
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::cmp::Ord;
	use std::fmt::Write;
	use std::cmp::PartialOrd;
	use std::convert::AsRef;
	use std::ops::Deref;
	use std::iter::IntoIterator;
	use std::iter::FromIterator;
	use std::default::Default;
	use std::cmp::PartialEq;
	use std::clone::Clone;
	use std::convert::AsMut;
	use buf::buf_impl::Buf;
	use std::convert::From;
	use std::borrow::BorrowMut;
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_0() {
    rusty_monitor::set_test_id(0);
    let mut bytesmut_0: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::default();
    let mut bytesmut_0_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_0;
    let mut isize_0: isize = -13223isize;
    let mut intoiter_0: crate::buf::iter::IntoIter<isize> = crate::buf::iter::IntoIter::new(isize_0);
    let mut intoiter_0_ref_0: &crate::buf::iter::IntoIter<isize> = &mut intoiter_0;
    let mut str_0: &str = "xJ78VDi69yo897r";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut usize_0: usize = 3662usize;
    let mut bytesmut_1: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::with_capacity(usize_0);
    let mut bytesmut_1_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_1;
    let mut usize_1: usize = 2289usize;
    let mut tuple_0: () = ();
    let mut result_0: std::result::Result<(), crate::TryGetError> = std::result::Result::Ok(tuple_0);
    let mut take_0: crate::buf::take::Take<std::result::Result<(), crate::TryGetError>> = crate::buf::take::Take {inner: result_0, limit: usize_1};
    let mut take_0_ref_0: &crate::buf::take::Take<std::result::Result<(), crate::TryGetError>> = &mut take_0;
    let mut usize_2: usize = 5712usize;
    let mut usize_3: usize = 4843usize;
    let mut vec_0: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut bytesmut_2: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from_iter::<std::vec::Vec<u8>>(vec_0);
    let mut bytesmut_2_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_2;
    let mut bytes_0: crate::bytes::Bytes = crate::bytes_mut::BytesMut::copy_to_bytes(bytesmut_2_ref_0, usize_3);
    let mut bytes_0_ref_0: &crate::bytes::Bytes = &mut bytes_0;
    let mut bytesmut_3: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::default();
    let mut usize_4: usize = 6783usize;
    let mut usize_5: usize = 2833usize;
    let mut bytes_1: crate::bytes::Bytes = crate::bytes::Bytes::from(bytesmut_3);
    let mut bytes_1_ref_0: &crate::bytes::Bytes = &mut bytes_1;
    let mut bool_0: bool = crate::bytes::Bytes::eq(bytes_1_ref_0, bytes_0_ref_0);
    crate::bytes_mut::BytesMut::write_str(bytesmut_1_ref_0, str_0_ref_0);
    let mut isize_1: &isize = crate::buf::iter::IntoIter::get_ref(intoiter_0_ref_0);
    panic!("From RustyUnit with love");
}

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_2() {
//     rusty_monitor::set_test_id(2);
//     let mut bytes_0: crate::bytes::Bytes = crate::bytes::Bytes::new();
//     let mut bytesmut_0: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from(bytes_0);
//     let mut bytesmut_0_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_0;
//     let mut usize_0: usize = 237usize;
//     let mut bytes_1: crate::bytes::Bytes = crate::bytes::Bytes::new();
//     let mut bytes_1_ref_0: &mut crate::bytes::Bytes = &mut bytes_1;
//     let mut usize_1: usize = 6747usize;
//     let mut usize_2: usize = 1779usize;
//     let mut trygeterror_0: crate::TryGetError = crate::TryGetError {requested: usize_2, available: usize_1};
//     let mut trygeterror_0_ref_0: &crate::TryGetError = &mut trygeterror_0;
//     let mut str_0: &str = "QoX8";
//     let mut str_0_ref_0: &str = &mut str_0;
//     let mut bytesmut_1: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from(str_0_ref_0);
//     let mut bytesmut_1_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_1;
//     let mut str_1: &str = "T9NS";
//     let mut str_1_ref_0: &str = &mut str_1;
//     let mut bytes_2: crate::bytes::Bytes = crate::bytes::Bytes::from(str_1_ref_0);
//     let mut bytesmut_2: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from(bytes_2);
//     let mut bytesmut_2_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_2;
//     let mut usize_3: usize = 6269usize;
//     let mut usize_4: usize = 5823usize;
//     let mut usize_5: usize = 8545usize;
//     let mut isize_0: isize = 627isize;
//     let mut take_0: crate::buf::take::Take<isize> = crate::buf::take::Take {inner: isize_0, limit: usize_5};
//     let mut take_0_ref_0: &mut crate::buf::take::Take<isize> = &mut take_0;
//     crate::buf::take::Take::set_limit(take_0_ref_0, usize_4);
//     let mut u8_slice_0: &mut [u8] = crate::bytes_mut::BytesMut::as_mut(bytesmut_1_ref_0);
//     let mut iter_0: std::slice::Iter<u8> = crate::bytes_mut::BytesMut::into_iter(bytesmut_2_ref_0);
//     crate::bytes::Bytes::advance(bytes_1_ref_0, usize_0);
//     let mut u8_slice_1: &mut [u8] = crate::bytes_mut::BytesMut::borrow_mut(bytesmut_0_ref_0);
//     panic!("From RustyUnit with love");
// }

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_6() {
//     rusty_monitor::set_test_id(6);
//     let mut bytesmut_0: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::default();
//     let mut bytes_0: crate::bytes::Bytes = crate::bytes::Bytes::from(bytesmut_0);
//     let mut bytes_0_ref_0: &crate::bytes::Bytes = &mut bytes_0;
//     let mut usize_0: usize = 4223usize;
//     let mut bytesmut_1: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::with_capacity(usize_0);
//     let mut bytes_1: crate::bytes::Bytes = crate::bytes::Bytes::from(bytesmut_1);
//     let mut bytes_1_ref_0: &crate::bytes::Bytes = &mut bytes_1;
//     let mut bytesmut_2: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::new();
//     let mut bytes_2: crate::bytes::Bytes = crate::bytes_mut::BytesMut::freeze(bytesmut_2);
//     let mut bytes_2_ref_0: &crate::bytes::Bytes = &mut bytes_2;
//     let mut u8_0: u8 = 22u8;
//     let mut usize_1: usize = 5805usize;
//     let mut bytes_3: crate::bytes::Bytes = crate::bytes::Bytes::new();
//     let mut bytesmut_3: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from(bytes_3);
//     let mut bytesmut_3_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_3;
//     let mut str_0: &str = "g59bNKHhGZ";
//     let mut str_0_ref_0: &str = &mut str_0;
//     let mut usize_2: usize = 8486usize;
//     let mut bytesmut_4: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::default();
//     let mut bytesmut_4_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_4;
//     let mut bytes_4: crate::bytes::Bytes = crate::bytes_mut::BytesMut::copy_to_bytes(bytesmut_4_ref_0, usize_2);
//     let mut bytes_4_ref_0: &crate::bytes::Bytes = &mut bytes_4;
//     let mut usize_3: usize = 7199usize;
//     let mut usize_4: usize = 6245usize;
//     let mut isize_0: isize = -1604isize;
//     let mut take_0: crate::buf::take::Take<isize> = crate::buf::take::Take {inner: isize_0, limit: usize_4};
//     let mut take_0_ref_0: &mut crate::buf::take::Take<isize> = &mut take_0;
//     let mut isize_1: isize = -1054isize;
//     let mut bytesmut_5: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::new();
//     let mut bytesmut_5_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_5;
//     let mut usize_5: usize = 8272usize;
//     let mut iter_0: std::slice::Iter<u8> = crate::bytes_mut::BytesMut::into_iter(bytesmut_5_ref_0);
//     let mut writer_0: crate::buf::writer::Writer<isize> = crate::buf::writer::new(isize_1);
//     let mut isize_2: &mut isize = crate::buf::take::Take::get_mut(take_0_ref_0);
//     let mut option_0: std::option::Option<std::cmp::Ordering> = crate::bytes::Bytes::partial_cmp(bytes_4_ref_0, str_0_ref_0);
//     crate::bytes_mut::BytesMut::resize(bytesmut_3_ref_0, usize_1, u8_0);
//     let mut ordering_0: std::cmp::Ordering = crate::bytes::Bytes::cmp(bytes_1_ref_0, bytes_0_ref_0);
//     panic!("From RustyUnit with love");
// }

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_9() {
    rusty_monitor::set_test_id(9);
    let mut bytesmut_0: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::default();
    let mut bytesmut_0_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_0;
    let mut bytesmut_1: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::new();
    let mut usize_0: usize = 18usize;
    let mut vec_0: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut bytes_0: crate::bytes::Bytes = crate::bytes::Bytes::from(vec_0);
    let mut bytes_0_ref_0: &crate::bytes::Bytes = &mut bytes_0;
    let mut usize_1: usize = 1589usize;
    let mut bytesmut_2: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::default();
    let mut bytesmut_2_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_2;
    let mut str_0: &str = "htHa8gcT";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytesmut_3: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from(str_0_ref_0);
    let mut bytesmut_3_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_3;
    let mut usize_2: usize = 6159usize;
    let mut isize_0: isize = 15328isize;
    let mut take_0: crate::buf::take::Take<isize> = crate::buf::take::new(isize_0, usize_2);
    let mut take_0_ref_0: &crate::buf::take::Take<isize> = &mut take_0;
    let mut usize_3: usize = 4424usize;
    let mut str_1: &str = "22gug";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytesmut_4: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from(str_1_ref_0);
    let mut bytesmut_4_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_4;
    crate::bytes_mut::BytesMut::advance(bytesmut_4_ref_0, usize_3);
    let mut isize_1: &isize = crate::buf::take::Take::get_ref(take_0_ref_0);
    let mut option_0: std::option::Option<std::cmp::Ordering> = crate::bytes_mut::BytesMut::partial_cmp(bytesmut_3_ref_0, bytesmut_2_ref_0);
    let mut ordering_0: std::cmp::Ordering = std::option::Option::unwrap(option_0);
    let mut bytes_1: crate::bytes::Bytes = crate::bytes::Bytes::clone(bytes_0_ref_0);
    let mut bytes_2: crate::bytes::Bytes = crate::bytes::Bytes::from(bytesmut_1);
    let mut u8_slice_0: &[u8] = crate::bytes_mut::BytesMut::as_ref(bytesmut_0_ref_0);
    panic!("From RustyUnit with love");
}

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_18() {
//     rusty_monitor::set_test_id(18);
//     let mut usize_0: usize = 3119usize;
//     let mut usize_1: usize = 8940usize;
//     let mut bytesmut_0: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::with_capacity(usize_1);
//     let mut bytesmut_0_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_0;
//     let mut bytes_0: crate::bytes::Bytes = crate::bytes_mut::BytesMut::copy_to_bytes(bytesmut_0_ref_0, usize_0);
//     let mut bytes_0_ref_0: &crate::bytes::Bytes = &mut bytes_0;
//     let mut usize_2: usize = 895usize;
//     let mut u16_0: u16 = 9889u16;
//     let mut result_0: std::result::Result<u16, crate::TryGetError> = std::result::Result::Ok(u16_0);
//     let mut take_0: crate::buf::take::Take<std::result::Result<u16, crate::TryGetError>> = crate::buf::take::Take {inner: result_0, limit: usize_2};
//     let mut take_0_ref_0: &crate::buf::take::Take<std::result::Result<u16, crate::TryGetError>> = &mut take_0;
//     let mut usize_3: usize = 1980usize;
//     let mut vec_0: std::vec::Vec<u8> = std::vec::Vec::new();
//     let mut vec_0_ref_0: &std::vec::Vec<u8> = &mut vec_0;
//     let mut usize_4: usize = 4314usize;
//     let mut box_0: std::boxed::Box<[u8]> = std::boxed::Box::new();
//     let mut bytes_1: crate::bytes::Bytes = crate::bytes::Bytes::from(box_0);
//     let mut bytes_1_ref_0: &mut crate::bytes::Bytes = &mut bytes_1;
//     let mut usize_5: usize = 8098usize;
//     let mut str_0: &str = "mQ15xSok0eL9yYTtlR3";
//     let mut string_0: std::string::String = std::string::String::from(str_0);
//     let mut bytes_2: crate::bytes::Bytes = crate::bytes::Bytes::from(string_0);
//     let mut bytes_2_ref_0: &mut crate::bytes::Bytes = &mut bytes_2;
//     let mut str_1: &str = "VDZpFKzQM4SXTgZP6";
//     let mut str_1_ref_0: &str = &mut str_1;
//     let mut usize_6: usize = 5732usize;
//     let mut bytesmut_1: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::zeroed(usize_6);
//     let mut bytesmut_1_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_1;
//     let mut option_0: std::option::Option<std::cmp::Ordering> = crate::bytes_mut::BytesMut::partial_cmp(bytesmut_1_ref_0, str_1_ref_0);
//     let mut ordering_0: std::cmp::Ordering = std::option::Option::unwrap(option_0);
//     let mut bytes_3: crate::bytes::Bytes = crate::bytes::Bytes::copy_to_bytes(bytes_2_ref_0, usize_5);
//     crate::bytes::Bytes::advance(bytes_1_ref_0, usize_4);
//     let mut bytes_3_ref_0: &crate::bytes::Bytes = &mut bytes_3;
//     let mut bool_0: bool = crate::bytes::Bytes::eq(bytes_3_ref_0, vec_0_ref_0);
//     let mut u8_slice_0: &[u8] = crate::bytes::Bytes::deref(bytes_0_ref_0);
//     panic!("From RustyUnit with love");
// }

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_29() {
//     rusty_monitor::set_test_id(29);
//     let mut bytes_0: crate::bytes::Bytes = crate::bytes::Bytes::new();
//     let mut bytes_0_ref_0: &crate::bytes::Bytes = &mut bytes_0;
//     let mut usize_0: usize = 3224usize;
//     let mut bytesmut_0: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::new();
//     let mut bytesmut_0_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_0;
//     let mut str_0: &str = "y";
//     let mut str_0_ref_0: &str = &mut str_0;
//     let mut bytesmut_1: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from(str_0_ref_0);
//     let mut bytesmut_1_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_1;
//     let mut bytesmut_2: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::new();
//     let mut bytes_1: crate::bytes::Bytes = crate::bytes::Bytes::from(bytesmut_2);
//     let mut bytes_1_ref_0: &crate::bytes::Bytes = &mut bytes_1;
//     let mut usize_1: usize = 7178usize;
//     let mut bytesmut_3: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::default();
//     let mut bytesmut_3_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_3;
//     let mut bytes_2: crate::bytes::Bytes = crate::bytes::Bytes::new();
//     let mut bytes_2_ref_0: &crate::bytes::Bytes = &mut bytes_2;
//     let mut usize_2: usize = 6307usize;
//     let mut usize_3: usize = 1901usize;
//     let mut usize_4: usize = 5836usize;
//     let mut usize_5: usize = 3691usize;
//     let mut trygeterror_0: crate::TryGetError = crate::TryGetError {requested: usize_5, available: usize_4};
//     let mut result_0: std::result::Result<(), crate::TryGetError> = std::result::Result::Err(trygeterror_0);
//     let mut take_0: crate::buf::take::Take<std::result::Result<(), crate::TryGetError>> = crate::buf::take::Take {inner: result_0, limit: usize_3};
//     let mut take_0_ref_0: &crate::buf::take::Take<std::result::Result<(), crate::TryGetError>> = &mut take_0;
//     let mut usize_6: usize = 6099usize;
//     let mut iter_0: std::slice::Iter<u8> = crate::bytes::Bytes::into_iter(bytes_2_ref_0);
//     let mut u8_slice_0: &mut [u8] = crate::bytes_mut::BytesMut::borrow_mut(bytesmut_3_ref_0);
//     let mut iter_1: std::slice::Iter<u8> = crate::bytes::Bytes::into_iter(bytes_1_ref_0);
//     let mut bytesmut_4: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::split(bytesmut_1_ref_0);
//     let mut bool_0: bool = crate::bytes_mut::BytesMut::is_empty(bytesmut_0_ref_0);
//     let mut u8_slice_1: &[u8] = crate::bytes::Bytes::deref(bytes_0_ref_0);
//     panic!("From RustyUnit with love");
// }

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_32() {
//     rusty_monitor::set_test_id(32);
//     let mut usize_0: usize = 7290usize;
//     let mut bytesmut_0: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::new();
//     let mut bytesmut_0_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_0;
//     let mut usize_1: usize = 851usize;
//     let mut u8_0: u8 = 85u8;
//     let mut usize_2: usize = 6797usize;
//     let mut bytesmut_1: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::default();
//     let mut bytesmut_1_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_1;
//     let mut bytes_0: crate::bytes::Bytes = crate::bytes::Bytes::default();
//     let mut bytes_0_ref_0: &crate::bytes::Bytes = &mut bytes_0;
//     let mut usize_3: usize = 7799usize;
//     let mut usize_4: usize = 551usize;
//     let mut isize_0: isize = 16253isize;
//     let mut take_0: crate::buf::take::Take<isize> = crate::buf::take::Take {inner: isize_0, limit: usize_4};
//     let mut take_0_ref_0: &mut crate::buf::take::Take<isize> = &mut take_0;
//     let mut usize_5: usize = 4028usize;
//     let mut vec_0: std::vec::Vec<u8> = std::vec::Vec::new();
//     let mut bytesmut_2: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from_iter::<std::vec::Vec<u8>>(vec_0);
//     let mut bytesmut_2_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_2;
//     let mut bytes_1: crate::bytes::Bytes = crate::bytes_mut::BytesMut::copy_to_bytes(bytesmut_2_ref_0, usize_5);
//     let mut bytes_1_ref_0: &crate::bytes::Bytes = &mut bytes_1;
//     let mut vec_1: std::vec::Vec<u8> = std::vec::Vec::new();
//     let mut bytesmut_3: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from_iter::<std::vec::Vec<u8>>(vec_1);
//     let mut bytesmut_3_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_3;
//     let mut iter_0: std::slice::Iter<u8> = crate::bytes_mut::BytesMut::into_iter(bytesmut_3_ref_0);
//     let mut usize_6: usize = crate::bytes::Bytes::remaining(bytes_1_ref_0);
//     crate::buf::take::Take::set_limit(take_0_ref_0, usize_3);
//     let mut bytes_2: crate::bytes::Bytes = crate::bytes::Bytes::clone(bytes_0_ref_0);
//     crate::bytes_mut::BytesMut::resize(bytesmut_1_ref_0, usize_2, u8_0);
//     let mut bytes_2_ref_0: &mut crate::bytes::Bytes = &mut bytes_2;
//     let mut bytes_3: crate::bytes::Bytes = crate::bytes::Bytes::copy_to_bytes(bytes_2_ref_0, usize_1);
//     crate::bytes_mut::BytesMut::advance(bytesmut_0_ref_0, usize_0);
//     panic!("From RustyUnit with love");
// }
}