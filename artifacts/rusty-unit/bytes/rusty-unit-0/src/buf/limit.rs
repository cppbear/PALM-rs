use crate::buf::UninitSlice;
use crate::BufMut;

use core::cmp;

/// A `BufMut` adapter which limits the amount of bytes that can be written
/// to an underlying buffer.
#[derive(Debug)]
pub struct Limit<T> {
    inner: T,
    limit: usize,
}

pub(super) fn new<T>(inner: T, limit: usize) -> Limit<T> {
    Limit { inner, limit }
}

impl<T> Limit<T> {
    /// Consumes this `Limit`, returning the underlying value.
    pub fn into_inner(self) -> T {
        self.inner
    }

    /// Gets a reference to the underlying `BufMut`.
    ///
    /// It is inadvisable to directly write to the underlying `BufMut`.
    pub fn get_ref(&self) -> &T {
        &self.inner
    }

    /// Gets a mutable reference to the underlying `BufMut`.
    ///
    /// It is inadvisable to directly write to the underlying `BufMut`.
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    /// Returns the maximum number of bytes that can be written
    ///
    /// # Note
    ///
    /// If the inner `BufMut` has fewer bytes than indicated by this method then
    /// that is the actual number of available bytes.
    pub fn limit(&self) -> usize {
        self.limit
    }

    /// Sets the maximum number of bytes that can be written.
    ///
    /// # Note
    ///
    /// If the inner `BufMut` has fewer bytes than `lim` then that is the actual
    /// number of available bytes.
    pub fn set_limit(&mut self, lim: usize) {
        self.limit = lim
    }
}

unsafe impl<T: BufMut> BufMut for Limit<T> {
    fn remaining_mut(&self) -> usize {
        cmp::min(self.inner.remaining_mut(), self.limit)
    }

    fn chunk_mut(&mut self) -> &mut UninitSlice {
        let bytes = self.inner.chunk_mut();
        let end = cmp::min(bytes.len(), self.limit);
        &mut bytes[..end]
    }

    unsafe fn advance_mut(&mut self, cnt: usize) {
        assert!(cnt <= self.limit);
        self.inner.advance_mut(cnt);
        self.limit -= cnt;
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::cmp::Ord;
	use std::cmp::PartialOrd;
	use std::convert::AsRef;
	use std::ops::Deref;
	use std::iter::Extend;
	use std::iter::IntoIterator;
	use std::iter::FromIterator;
	use std::default::Default;
	use std::clone::Clone;
	use std::cmp::PartialEq;
	use buf::buf_impl::Buf;
	use std::convert::From;
	use std::cmp::Eq;
	use std::borrow::BorrowMut;
	use std::borrow::Borrow;
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5() {
    rusty_monitor::set_test_id(5);
    let mut usize_0: usize = 5737usize;
    let mut usize_1: usize = 2577usize;
    let mut trygeterror_0: crate::TryGetError = crate::TryGetError {requested: usize_1, available: usize_0};
    let mut result_0: std::result::Result<u16, crate::TryGetError> = std::result::Result::Err(trygeterror_0);
    let mut intoiter_0: crate::buf::iter::IntoIter<std::result::Result<u16, crate::TryGetError>> = crate::buf::iter::IntoIter::new(result_0);
    let mut intoiter_0_ref_0: &crate::buf::iter::IntoIter<std::result::Result<u16, crate::TryGetError>> = &mut intoiter_0;
    let mut bytesmut_0: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::default();
    let mut bytesmut_0_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_0;
    let mut bytes_0: crate::bytes::Bytes = crate::bytes::Bytes::new();
    let mut bytesmut_1: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from(bytes_0);
    let mut bytesmut_1_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_1;
    let mut usize_2: usize = 6882usize;
    let mut str_0: &str = "Zd9VA";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytesmut_2: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from(str_0_ref_0);
    let mut bytesmut_2_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_2;
    let mut str_1: &str = "nEbI0KY";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut usize_3: usize = 6265usize;
    let mut bytesmut_3: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::with_capacity(usize_3);
    let mut bytes_1: crate::bytes::Bytes = crate::bytes_mut::BytesMut::freeze(bytesmut_3);
    let mut bytes_1_ref_0: &crate::bytes::Bytes = &mut bytes_1;
    let mut usize_4: usize = 8991usize;
    let mut usize_5: usize = 7320usize;
    let mut isize_0: isize = 6784isize;
    let mut limit_0: crate::buf::limit::Limit<isize> = crate::buf::limit::Limit {inner: isize_0, limit: usize_5};
    let mut limit_0_ref_0: &mut crate::buf::limit::Limit<isize> = &mut limit_0;
    crate::buf::limit::Limit::set_limit(limit_0_ref_0, usize_4);
    let mut bytes_2: crate::bytes::Bytes = crate::bytes::Bytes::clone(bytes_1_ref_0);
    let mut bytes_2_ref_0: &crate::bytes::Bytes = &mut bytes_2;
    let mut bool_0: bool = crate::bytes::Bytes::eq(bytes_2_ref_0, str_1_ref_0);
    let mut u8_slice_0: &[u8] = crate::bytes_mut::BytesMut::deref(bytesmut_2_ref_0);
    panic!("From RustyUnit with love");
}

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_8() {
//     rusty_monitor::set_test_id(8);
//     let mut usize_0: usize = 821usize;
//     let mut bytesmut_0: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::zeroed(usize_0);
//     let mut bytesmut_0_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_0;
//     let mut usize_1: usize = 8873usize;
//     let mut bytesmut_1: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::with_capacity(usize_1);
//     let mut bytesmut_1_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_1;
//     let mut usize_2: usize = 6415usize;
//     let mut usize_3: usize = 7687usize;
//     let mut isize_0: isize = -3913isize;
//     let mut limit_0: crate::buf::limit::Limit<isize> = crate::buf::limit::Limit {inner: isize_0, limit: usize_3};
//     let mut limit_0_ref_0: &mut crate::buf::limit::Limit<isize> = &mut limit_0;
//     let mut usize_4: usize = 8249usize;
//     let mut usize_5: usize = 9497usize;
//     let mut trygeterror_0: crate::TryGetError = crate::TryGetError {requested: usize_5, available: usize_4};
//     let mut result_0: std::result::Result<u16, crate::TryGetError> = std::result::Result::Err(trygeterror_0);
//     let mut intoiter_0: crate::buf::iter::IntoIter<std::result::Result<u16, crate::TryGetError>> = crate::buf::iter::IntoIter::new(result_0);
//     let mut intoiter_0_ref_0: &crate::buf::iter::IntoIter<std::result::Result<u16, crate::TryGetError>> = &mut intoiter_0;
//     let mut bytesmut_2: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::new();
//     let mut bytesmut_2_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_2;
//     let mut usize_6: usize = 9595usize;
//     let mut isize_1: isize = -23881isize;
//     let mut limit_1: crate::buf::limit::Limit<isize> = crate::buf::limit::new(isize_1, usize_6);
//     let mut limit_1_ref_0: &crate::buf::limit::Limit<isize> = &mut limit_1;
//     let mut usize_7: usize = crate::buf::limit::Limit::limit(limit_1_ref_0);
//     let mut bytesmut_3: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::default();
//     let mut bytesmut_3_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_3;
//     let mut iter_0: std::slice::Iter<u8> = crate::bytes_mut::BytesMut::into_iter(bytesmut_2_ref_0);
//     let mut isize_2: &mut isize = crate::buf::limit::Limit::get_mut(limit_0_ref_0);
//     let mut iter_1: std::slice::Iter<u8> = crate::bytes_mut::BytesMut::into_iter(bytesmut_3_ref_0);
//     let mut bytes_0: crate::bytes::Bytes = crate::bytes::Bytes::new();
//     let mut u8_slice_0: &[u8] = crate::bytes_mut::BytesMut::as_ref(bytesmut_1_ref_0);
//     let mut u8_slice_1: &mut [u8] = crate::bytes_mut::BytesMut::borrow_mut(bytesmut_0_ref_0);
//     panic!("From RustyUnit with love");
// }

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_12() {
    rusty_monitor::set_test_id(12);
    let mut bytesmut_0: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::default();
    let mut bytesmut_0_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_0;
    let mut usize_0: usize = 9753usize;
    let mut str_0: &str = "lsoMk2II3F";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytesmut_1: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from(str_0_ref_0);
    let mut bytesmut_1_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_1;
    let mut bytes_0: crate::bytes::Bytes = crate::bytes_mut::BytesMut::copy_to_bytes(bytesmut_1_ref_0, usize_0);
    let mut bytes_0_ref_0: &crate::bytes::Bytes = &mut bytes_0;
    let mut usize_1: usize = 3154usize;
    let mut bytesmut_2: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::with_capacity(usize_1);
    let mut bytesmut_2_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_2;
    let mut bytesmut_3: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::new();
    let mut bytesmut_3_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_3;
    let mut usize_2: usize = 6258usize;
    let mut bytesmut_4: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::zeroed(usize_2);
    let mut bytesmut_4_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_4;
    let mut str_1: &str = "";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytesmut_5: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from(str_1_ref_0);
    let mut bytesmut_5_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_5;
    let mut usize_3: usize = 835usize;
    let mut usize_4: usize = 7508usize;
    let mut isize_0: isize = 4514isize;
    let mut limit_0: crate::buf::limit::Limit<isize> = crate::buf::limit::Limit {inner: isize_0, limit: usize_4};
    let mut u8_slice_0: &[u8] = crate::bytes_mut::BytesMut::chunk(bytesmut_5_ref_0);
    let mut limit_0_ref_0: &crate::buf::limit::Limit<isize> = &mut limit_0;
    let mut isize_1: &isize = crate::buf::limit::Limit::get_ref(limit_0_ref_0);
    let mut ordering_0: std::cmp::Ordering = crate::bytes_mut::BytesMut::cmp(bytesmut_4_ref_0, bytesmut_3_ref_0);
    let mut bytes_1: crate::bytes::Bytes = crate::bytes::Bytes::clone(bytes_0_ref_0);
    let mut bytes_1_ref_0: &crate::bytes::Bytes = &mut bytes_1;
    let mut bool_0: bool = crate::bytes::Bytes::eq(bytes_1_ref_0, bytesmut_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_26() {
    rusty_monitor::set_test_id(26);
    let mut bytes_0: crate::bytes::Bytes = crate::bytes::Bytes::new();
    let mut bytes_0_ref_0: &crate::bytes::Bytes = &mut bytes_0;
    let mut usize_0: usize = 1416usize;
    let mut bytesmut_0: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::with_capacity(usize_0);
    let mut bytes_1: crate::bytes::Bytes = crate::bytes::Bytes::from(bytesmut_0);
    let mut bytes_1_ref_0: &crate::bytes::Bytes = &mut bytes_1;
    let mut usize_1: usize = 8501usize;
    let mut usize_2: usize = 188usize;
    let mut usize_3: usize = 5793usize;
    let mut isize_0: isize = -228isize;
    let mut limit_0: crate::buf::limit::Limit<isize> = crate::buf::limit::Limit {inner: isize_0, limit: usize_3};
    let mut limit_0_ref_0: &mut crate::buf::limit::Limit<isize> = &mut limit_0;
    let mut usize_4: usize = 790usize;
    let mut bytesmut_1: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::with_capacity(usize_4);
    let mut bytesmut_1_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_1;
    let mut usize_5: usize = 1105usize;
    let mut usize_6: usize = 5952usize;
    let mut usize_7: usize = 11usize;
    let mut str_0: &str = "IQRPnSjH";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytesmut_2: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from(str_0_ref_0);
    let mut bytesmut_2_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_2;
    let mut usize_8: usize = 6858usize;
    let mut usize_9: usize = 1818usize;
    let mut isize_1: isize = 968isize;
    let mut limit_1: crate::buf::limit::Limit<isize> = crate::buf::limit::Limit {inner: isize_1, limit: usize_9};
    let mut isize_2: isize = crate::buf::limit::Limit::into_inner(limit_1);
    let mut bytesmut_3: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::zeroed(usize_8);
    crate::bytes_mut::BytesMut::reserve(bytesmut_2_ref_0, usize_7);
    let mut bytesmut_3_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_3;
    let mut bool_0: bool = crate::bytes_mut::BytesMut::eq(bytesmut_3_ref_0, bytesmut_1_ref_0);
    crate::buf::limit::Limit::set_limit(limit_0_ref_0, usize_2);
    let mut option_0: std::option::Option<std::cmp::Ordering> = crate::bytes::Bytes::partial_cmp(bytes_1_ref_0, bytes_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_28() {
    rusty_monitor::set_test_id(28);
    let mut usize_0: usize = 5765usize;
    let mut usize_1: usize = 9938usize;
    let mut trygeterror_0: crate::TryGetError = crate::TryGetError {requested: usize_1, available: usize_0};
    let mut trygeterror_0_ref_0: &crate::TryGetError = &mut trygeterror_0;
    let mut usize_2: usize = 8797usize;
    let mut usize_3: usize = 5255usize;
    let mut trygeterror_1: crate::TryGetError = crate::TryGetError {requested: usize_3, available: usize_2};
    let mut trygeterror_1_ref_0: &crate::TryGetError = &mut trygeterror_1;
    let mut usize_4: usize = 1923usize;
    let mut isize_0: isize = 8322isize;
    let mut bytesmut_0: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::new();
    let mut bytes_0: crate::bytes::Bytes = crate::bytes::Bytes::from(bytesmut_0);
    let mut bytes_0_ref_0: &crate::bytes::Bytes = &mut bytes_0;
    let mut usize_5: usize = 163usize;
    let mut usize_6: usize = 750usize;
    let mut str_0: &str = "kpXgoApmRDDiEOe5";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut usize_7: usize = 4854usize;
    let mut bytesmut_1: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::with_capacity(usize_7);
    let mut bytesmut_1_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_1;
    let mut usize_8: usize = 5194usize;
    let mut usize_9: usize = 5280usize;
    let mut usize_10: usize = 1030usize;
    let mut usize_11: usize = 3527usize;
    let mut isize_1: isize = -5409isize;
    let mut limit_0: crate::buf::limit::Limit<isize> = crate::buf::limit::Limit {inner: isize_1, limit: usize_11};
    let mut limit_0_ref_0: &mut crate::buf::limit::Limit<isize> = &mut limit_0;
    crate::buf::limit::Limit::set_limit(limit_0_ref_0, usize_10);
    let mut option_0: std::option::Option<std::cmp::Ordering> = crate::bytes_mut::BytesMut::partial_cmp(bytesmut_1_ref_0, str_0_ref_0);
    let mut u8_slice_0: &[u8] = crate::bytes::Bytes::borrow(bytes_0_ref_0);
    let mut ordering_0: std::cmp::Ordering = std::option::Option::unwrap(option_0);
    let mut limit_1: crate::buf::limit::Limit<isize> = crate::buf::limit::Limit {inner: isize_0, limit: usize_4};
    let mut bool_0: bool = crate::TryGetError::eq(trygeterror_1_ref_0, trygeterror_0_ref_0);
    panic!("From RustyUnit with love");
}

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_31() {
//     rusty_monitor::set_test_id(31);
//     let mut bytesmut_0: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::default();
//     let mut bytes_0: crate::bytes::Bytes = crate::bytes::Bytes::from(bytesmut_0);
//     let mut bytes_0_ref_0: &crate::bytes::Bytes = &mut bytes_0;
//     let mut bytesmut_1: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::new();
//     let mut bytes_1: crate::bytes::Bytes = crate::bytes_mut::BytesMut::freeze(bytesmut_1);
//     let mut bytes_1_ref_0: &crate::bytes::Bytes = &mut bytes_1;
//     let mut usize_0: usize = 9030usize;
//     let mut bytesmut_2: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::zeroed(usize_0);
//     let mut bytesmut_2_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_2;
//     let mut bytesmut_3: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::new();
//     let mut bytesmut_3_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_3;
//     let mut usize_1: usize = 2561usize;
//     let mut bytesmut_4: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::default();
//     let mut bytesmut_4_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_4;
//     let mut bytesmut_5: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::default();
//     let mut bytesmut_5_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_5;
//     let mut usize_2: usize = 855usize;
//     let mut isize_0: isize = 13052isize;
//     let mut limit_0: crate::buf::limit::Limit<isize> = crate::buf::limit::new(isize_0, usize_2);
//     let mut usize_3: usize = 6570usize;
//     let mut usize_4: usize = 7207usize;
//     let mut trygeterror_0: crate::TryGetError = crate::TryGetError {requested: usize_4, available: usize_3};
//     let mut trygeterror_0_ref_0: &crate::TryGetError = &mut trygeterror_0;
//     let mut vec_0: std::vec::Vec<u8> = std::vec::Vec::new();
//     let mut bytesmut_6: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::default();
//     let mut bytesmut_6_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_6;
//     crate::bytes_mut::BytesMut::extend(bytesmut_6_ref_0, vec_0);
//     let mut isize_1: isize = crate::buf::limit::Limit::into_inner(limit_0);
//     let mut iter_0: std::slice::Iter<u8> = crate::bytes_mut::BytesMut::into_iter(bytesmut_5_ref_0);
//     let mut option_0: std::option::Option<std::cmp::Ordering> = crate::bytes_mut::BytesMut::partial_cmp(bytesmut_3_ref_0, bytesmut_2_ref_0);
//     let mut iter_1: std::slice::Iter<u8> = crate::bytes::Bytes::into_iter(bytes_1_ref_0);
//     let mut usize_5: usize = crate::bytes::Bytes::remaining(bytes_0_ref_0);
//     panic!("From RustyUnit with love");
// }

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_33() {
    rusty_monitor::set_test_id(33);
    let mut usize_0: usize = 5667usize;
    let mut usize_1: usize = 9619usize;
    let mut isize_0: isize = -1656isize;
    let mut limit_0: crate::buf::limit::Limit<isize> = crate::buf::limit::Limit {inner: isize_0, limit: usize_1};
    let mut limit_0_ref_0: &mut crate::buf::limit::Limit<isize> = &mut limit_0;
    let mut usize_2: usize = 4052usize;
    let mut usize_3: usize = 8344usize;
    let mut bytesmut_0: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::with_capacity(usize_3);
    let mut bytesmut_0_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_0;
    let mut vec_0: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_1: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut bytesmut_1: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from_iter::<std::vec::Vec<u8>>(vec_1);
    let mut bytesmut_1_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_1;
    let mut vec_2: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut bytesmut_2: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from_iter::<std::vec::Vec<u8>>(vec_2);
    let mut str_0: &str = "x3rX";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytesmut_3: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from(str_0_ref_0);
    let mut bytesmut_3_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_3;
    let mut usize_4: usize = 7142usize;
    let mut bytesmut_4: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::default();
    let mut bytesmut_4_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_4;
    let mut bytes_0: crate::bytes::Bytes = crate::bytes_mut::BytesMut::copy_to_bytes(bytesmut_4_ref_0, usize_4);
    let mut bytes_0_ref_0: &crate::bytes::Bytes = &mut bytes_0;
    let mut usize_5: usize = 9546usize;
    let mut bytesmut_5: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::zeroed(usize_5);
    let mut bytesmut_5_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_5;
    let mut usize_6: usize = 9714usize;
    let mut bool_0: bool = crate::bytes_mut::BytesMut::eq(bytesmut_5_ref_0, bytes_0_ref_0);
    crate::bytes_mut::BytesMut::unsplit(bytesmut_3_ref_0, bytesmut_2);
    crate::bytes_mut::BytesMut::extend(bytesmut_1_ref_0, vec_0);
    crate::bytes_mut::BytesMut::reserve(bytesmut_0_ref_0, usize_2);
    crate::buf::limit::Limit::set_limit(limit_0_ref_0, usize_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_41() {
    rusty_monitor::set_test_id(41);
    let mut usize_0: usize = 2125usize;
    let mut bytesmut_0: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::with_capacity(usize_0);
    let mut bytes_0: crate::bytes::Bytes = crate::bytes::Bytes::from(bytesmut_0);
    let mut bytes_0_ref_0: &crate::bytes::Bytes = &mut bytes_0;
    let mut str_0: &str = "nVw7EE";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytesmut_1: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from(str_0_ref_0);
    let mut bytes_1: crate::bytes::Bytes = crate::bytes_mut::BytesMut::freeze(bytesmut_1);
    let mut bytes_1_ref_0: &crate::bytes::Bytes = &mut bytes_1;
    let mut usize_1: usize = 4284usize;
    let mut usize_2: usize = 623usize;
    let mut trygeterror_0: crate::TryGetError = crate::TryGetError {requested: usize_2, available: usize_1};
    let mut trygeterror_0_ref_0: &crate::TryGetError = &mut trygeterror_0;
    let mut usize_3: usize = 8073usize;
    let mut bytesmut_2: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::zeroed(usize_3);
    let mut bytesmut_2_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_2;
    let mut usize_4: usize = 7820usize;
    let mut bytes_2: crate::bytes::Bytes = crate::bytes::Bytes::default();
    let mut bytesmut_3: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from(bytes_2);
    let mut bytesmut_3_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_3;
    let mut vec_0: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_0_ref_0: &std::vec::Vec<u8> = &mut vec_0;
    let mut bytes_3: crate::bytes::Bytes = crate::bytes::Bytes::default();
    let mut bytesmut_4: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from(bytes_3);
    let mut bytesmut_4_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_4;
    let mut usize_5: usize = 5085usize;
    let mut bool_0: bool = crate::bytes_mut::BytesMut::eq(bytesmut_4_ref_0, vec_0_ref_0);
    let mut usize_6: usize = crate::bytes_mut::BytesMut::remaining(bytesmut_3_ref_0);
    let mut u8_slice_0: &[u8] = crate::bytes_mut::BytesMut::deref(bytesmut_2_ref_0);
    let mut tuple_0: () = crate::TryGetError::assert_receiver_is_total_eq(trygeterror_0_ref_0);
    let mut u8_slice_1: &[u8] = crate::bytes::Bytes::borrow(bytes_1_ref_0);
    panic!("From RustyUnit with love");
}

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_44() {
//     rusty_monitor::set_test_id(44);
//     let mut usize_0: usize = 6785usize;
//     let mut isize_0: isize = 1751isize;
//     let mut limit_0: crate::buf::limit::Limit<isize> = crate::buf::limit::Limit {inner: isize_0, limit: usize_0};
//     let mut limit_0_ref_0: &mut crate::buf::limit::Limit<isize> = &mut limit_0;
//     let mut str_0: &str = "ni7qJXhtBCP";
//     let mut str_0_ref_0: &str = &mut str_0;
//     let mut bytesmut_0: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from(str_0_ref_0);
//     let mut bytesmut_0_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_0;
//     let mut usize_1: usize = 9161usize;
//     let mut vec_0: std::vec::Vec<u8> = std::vec::Vec::new();
//     let mut bytesmut_1: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from_iter::<std::vec::Vec<u8>>(vec_0);
//     let mut bytesmut_1_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_1;
//     let mut str_1: &str = "nlq";
//     let mut str_1_ref_0: &str = &mut str_1;
//     let mut bytes_0: crate::bytes::Bytes = crate::bytes::Bytes::from(str_1_ref_0);
//     let mut bytesmut_2: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from(bytes_0);
//     let mut bytesmut_2_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_2;
//     let mut box_0: std::boxed::Box<[u8]> = std::boxed::Box::new();
//     let mut bytes_1: crate::bytes::Bytes = crate::bytes::Bytes::from(box_0);
//     let mut bytesmut_3: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from(bytes_1);
//     let mut bytesmut_3_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_3;
//     let mut usize_2: usize = 9197usize;
//     let mut usize_3: usize = 1426usize;
//     let mut usize_4: usize = 2355usize;
//     let mut vec_1: std::vec::Vec<u8> = std::vec::Vec::new();
//     let mut bytesmut_4: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from_iter::<std::vec::Vec<u8>>(vec_1);
//     let mut bytesmut_4_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_4;
//     let mut bytes_2: crate::bytes::Bytes = crate::bytes_mut::BytesMut::copy_to_bytes(bytesmut_4_ref_0, usize_4);
//     let mut bytes_2_ref_0: &mut crate::bytes::Bytes = &mut bytes_2;
//     crate::bytes::Bytes::advance(bytes_2_ref_0, usize_3);
//     let mut option_0: std::option::Option<std::cmp::Ordering> = crate::bytes_mut::BytesMut::partial_cmp(bytesmut_3_ref_0, bytesmut_2_ref_0);
//     let mut bytes_3: crate::bytes::Bytes = crate::bytes_mut::BytesMut::copy_to_bytes(bytesmut_1_ref_0, usize_1);
//     let mut iter_0: std::slice::Iter<u8> = crate::bytes_mut::BytesMut::into_iter(bytesmut_0_ref_0);
//     let mut isize_1: &mut isize = crate::buf::limit::Limit::get_mut(limit_0_ref_0);
//     panic!("From RustyUnit with love");
// }

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_47() {
    rusty_monitor::set_test_id(47);
    let mut str_0: &str = "v9IU5";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytes_0: crate::bytes::Bytes = crate::bytes::Bytes::from(str_0_ref_0);
    let mut bytes_0_ref_0: &crate::bytes::Bytes = &mut bytes_0;
    let mut bytes_1: crate::bytes::Bytes = crate::bytes::Bytes::default();
    let mut bytes_1_ref_0: &crate::bytes::Bytes = &mut bytes_1;
    let mut u128_0: u128 = 8882u128;
    let mut result_0: std::result::Result<u128, crate::TryGetError> = std::result::Result::Ok(u128_0);
    let mut intoiter_0: crate::buf::iter::IntoIter<std::result::Result<u128, crate::TryGetError>> = crate::buf::iter::IntoIter::new(result_0);
    let mut intoiter_0_ref_0: &crate::buf::iter::IntoIter<std::result::Result<u128, crate::TryGetError>> = &mut intoiter_0;
    let mut usize_0: usize = 8742usize;
    let mut str_1: &str = "uB8p7WwwIy";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytesmut_0: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from(str_1_ref_0);
    let mut bytesmut_0_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_0;
    let mut bytes_2: crate::bytes::Bytes = crate::bytes_mut::BytesMut::copy_to_bytes(bytesmut_0_ref_0, usize_0);
    let mut bytesmut_1: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::new();
    let mut bytesmut_1_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_1;
    let mut isize_0: isize = -817isize;
    let mut bytesmut_2: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::default();
    let mut bytesmut_2_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_2;
    let mut bytesmut_3: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::default();
    let mut bytes_3: crate::bytes::Bytes = crate::bytes::Bytes::from(bytesmut_3);
    let mut bytes_3_ref_0: &crate::bytes::Bytes = &mut bytes_3;
    let mut usize_1: usize = 3089usize;
    let mut isize_1: isize = -6869isize;
    let mut limit_0: crate::buf::limit::Limit<isize> = crate::buf::limit::new(isize_1, usize_1);
    let mut reader_0: crate::buf::reader::Reader<isize> = crate::buf::reader::new(isize_0);
    let mut u8_slice_0: &[u8] = crate::bytes_mut::BytesMut::borrow(bytesmut_1_ref_0);
    let mut bytesmut_4: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from(bytes_2);
    panic!("From RustyUnit with love");
}
}