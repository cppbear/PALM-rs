use alloc::collections::VecDeque;
#[cfg(feature = "std")]
use std::io;

use super::Buf;

impl Buf for VecDeque<u8> {
    fn remaining(&self) -> usize {
        self.len()
    }

    fn chunk(&self) -> &[u8] {
        let (s1, s2) = self.as_slices();
        if s1.is_empty() {
            s2
        } else {
            s1
        }
    }

    #[cfg(feature = "std")]
    fn chunks_vectored<'a>(&'a self, dst: &mut [io::IoSlice<'a>]) -> usize {
        if self.is_empty() || dst.is_empty() {
            return 0;
        }

        let (s1, s2) = self.as_slices();
        dst[0] = io::IoSlice::new(s1);
        if s2.is_empty() || dst.len() == 1 {
            return 1;
        }

        dst[1] = io::IoSlice::new(s2);
        2
    }

    fn advance(&mut self, cnt: usize) {
        self.drain(..cnt);
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::iter::IntoIterator;
	use std::iter::FromIterator;
	use std::convert::AsRef;
	use std::convert::From;
	use std::ops::Deref;
	use std::cmp::Eq;
// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_37() {
//     rusty_monitor::set_test_id(37);
//     let mut vec_0: std::vec::Vec<u8> = std::vec::Vec::new();
//     let mut bytes_0: crate::bytes::Bytes = crate::bytes::Bytes::from_iter::<std::vec::Vec<u8>>(vec_0);
//     let mut bytes_0_ref_0: &crate::bytes::Bytes = &mut bytes_0;
//     let mut usize_0: usize = 6802usize;
//     let mut usize_1: usize = 807usize;
//     let mut usize_2: usize = 9559usize;
//     let mut trygeterror_0: crate::TryGetError = crate::TryGetError {requested: usize_2, available: usize_1};
//     let mut trygeterror_0_ref_0: &crate::TryGetError = &mut trygeterror_0;
//     let mut usize_3: usize = 111usize;
//     let mut bytesmut_0: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::with_capacity(usize_3);
//     let mut bytesmut_0_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_0;
//     let mut usize_4: usize = 1423usize;
//     let mut bytesmut_1: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::with_capacity(usize_4);
//     let mut bytes_1: crate::bytes::Bytes = crate::bytes_mut::BytesMut::freeze(bytesmut_1);
//     let mut bytes_1_ref_0: &crate::bytes::Bytes = &mut bytes_1;
//     let mut usize_5: usize = 1042usize;
//     let mut vec_1: std::vec::Vec<u8> = std::vec::Vec::new();
//     let mut bytesmut_2: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from_iter::<std::vec::Vec<u8>>(vec_1);
//     let mut bytesmut_2_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_2;
//     let mut box_0: std::boxed::Box<[u8]> = std::boxed::Box::new();
//     let mut bytes_2: crate::bytes::Bytes = crate::bytes::Bytes::from(box_0);
//     let mut bytesmut_3: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from(bytes_2);
//     let mut bytesmut_3_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_3;
//     let mut usize_6: usize = 7903usize;
//     let mut box_1: std::boxed::Box<[u8]> = std::boxed::Box::new();
//     let mut take_0: crate::buf::take::Take<std::boxed::Box<[u8]>> = crate::buf::take::new(box_1, usize_6);
//     let mut take_0_ref_0: &crate::buf::take::Take<std::boxed::Box<[u8]>> = &mut take_0;
//     let mut u8_slice_0: &[u8] = crate::bytes_mut::BytesMut::as_ref(bytesmut_3_ref_0);
//     let mut u8_slice_1: &[u8] = crate::bytes_mut::BytesMut::as_ref(bytesmut_2_ref_0);
//     let mut u8_slice_2: &[u8] = crate::bytes::Bytes::deref(bytes_1_ref_0);
//     let mut iter_0: std::slice::Iter<u8> = crate::bytes_mut::BytesMut::into_iter(bytesmut_0_ref_0);
//     let mut tuple_0: () = crate::TryGetError::assert_receiver_is_total_eq(trygeterror_0_ref_0);
//     panic!("From RustyUnit with love");
// }
}