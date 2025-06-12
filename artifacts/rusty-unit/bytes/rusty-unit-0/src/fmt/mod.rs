macro_rules! fmt_impl {
    ($tr:ident, $ty:ty) => {
        impl $tr for $ty {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                $tr::fmt(&BytesRef(self.as_ref()), f)
            }
        }
    };
}

mod debug;
mod hex;

/// `BytesRef` is not a part of public API of bytes crate.
struct BytesRef<'a>(&'a [u8]);

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::iter::IntoIterator;
	use std::iter::FromIterator;
	use std::cmp::PartialEq;
	use buf::buf_impl::Buf;
	use std::convert::From;
	use std::cmp::Eq;
// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_13() {
//     rusty_monitor::set_test_id(13);
//     let mut usize_0: usize = 639usize;
//     let mut usize_1: usize = 8718usize;
//     let mut trygeterror_0: crate::TryGetError = crate::TryGetError {requested: usize_1, available: usize_0};
//     let mut trygeterror_0_ref_0: &crate::TryGetError = &mut trygeterror_0;
//     let mut usize_2: usize = 6209usize;
//     let mut bytesmut_0: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::zeroed(usize_2);
//     let mut bytesmut_0_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_0;
//     let mut usize_3: usize = 7655usize;
//     let mut vec_0: std::vec::Vec<u8> = std::vec::Vec::new();
//     let mut bytesmut_1: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from_iter::<std::vec::Vec<u8>>(vec_0);
//     let mut bytesmut_1_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_1;
//     let mut vec_1: std::vec::Vec<u8> = std::vec::Vec::new();
//     let mut vec_1_ref_0: &std::vec::Vec<u8> = &mut vec_1;
//     let mut usize_4: usize = 2228usize;
//     let mut bytesmut_2: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::zeroed(usize_4);
//     let mut bytesmut_2_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_2;
//     let mut usize_5: usize = 1217usize;
//     let mut vec_2: std::vec::Vec<u8> = std::vec::Vec::new();
//     let mut bytesmut_3: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from_iter::<std::vec::Vec<u8>>(vec_2);
//     let mut bytesmut_3_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_3;
//     let mut str_0: &str = "bTDXmA4X";
//     let mut string_0: std::string::String = std::string::String::from(str_0);
//     let mut bytes_0: crate::bytes::Bytes = crate::bytes::Bytes::from(string_0);
//     let mut bytes_0_ref_0: &crate::bytes::Bytes = &mut bytes_0;
//     let mut usize_6: usize = 8712usize;
//     let mut bytesmut_4: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::zeroed(usize_6);
//     let mut intoiter_0: crate::buf::iter::IntoIter<crate::bytes_mut::BytesMut> = crate::bytes_mut::BytesMut::into_iter(bytesmut_4);
//     let mut intoiter_0_ref_0: &mut crate::buf::iter::IntoIter<isize> = &mut intoiter_0;
//     let mut isize_0: &mut isize = crate::buf::iter::IntoIter::get_mut(intoiter_0_ref_0);
//     let mut bool_0: bool = crate::bytes::Bytes::eq(bytes_0_ref_0, bytesmut_3_ref_0);
//     let mut bool_1: bool = crate::bytes_mut::BytesMut::eq(bytesmut_2_ref_0, vec_1_ref_0);
//     crate::bytes_mut::BytesMut::advance(bytesmut_1_ref_0, usize_3);
//     let mut bool_2: bool = crate::bytes_mut::BytesMut::is_empty(bytesmut_0_ref_0);
//     let mut tuple_0: () = crate::TryGetError::assert_receiver_is_total_eq(trygeterror_0_ref_0);
//     panic!("From RustyUnit with love");
// }
}