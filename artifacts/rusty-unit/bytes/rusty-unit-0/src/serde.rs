use super::{Bytes, BytesMut};
use alloc::string::String;
use alloc::vec::Vec;
use core::{cmp, fmt};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

macro_rules! serde_impl {
    ($ty:ident, $visitor_ty:ident, $from_slice:ident, $from_vec:ident) => {
        impl Serialize for $ty {
            #[inline]
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_bytes(&self)
            }
        }

        struct $visitor_ty;

        impl<'de> de::Visitor<'de> for $visitor_ty {
            type Value = $ty;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("byte array")
            }

            #[inline]
            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: de::SeqAccess<'de>,
            {
                let len = cmp::min(seq.size_hint().unwrap_or(0), 4096);
                let mut values: Vec<u8> = Vec::with_capacity(len);

                while let Some(value) = seq.next_element()? {
                    values.push(value);
                }

                Ok($ty::$from_vec(values))
            }

            #[inline]
            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok($ty::$from_slice(v))
            }

            #[inline]
            fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok($ty::$from_vec(v))
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok($ty::$from_slice(v.as_bytes()))
            }

            #[inline]
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok($ty::$from_vec(v.into_bytes()))
            }
        }

        impl<'de> Deserialize<'de> for $ty {
            #[inline]
            fn deserialize<D>(deserializer: D) -> Result<$ty, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_byte_buf($visitor_ty)
            }
        }
    };
}

serde_impl!(Bytes, BytesVisitor, copy_from_slice, from);
serde_impl!(BytesMut, BytesMutVisitor, from, from_vec);

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::iter::FromIterator;
	use std::default::Default;
	use buf::buf_impl::Buf;
	use std::cmp::PartialOrd;
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_15() {
    rusty_monitor::set_test_id(15);
    let mut bytesmut_0: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::default();
    let mut usize_0: usize = 5227usize;
    let mut bytesmut_1: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::default();
    let mut bytesmut_1_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_1;
    let mut usize_1: usize = 8215usize;
    let mut vec_0: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut bytesmut_2: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from_iter::<std::vec::Vec<u8>>(vec_0);
    let mut bytesmut_2_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_2;
    let mut usize_2: usize = 7529usize;
    let mut bytesmut_3: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::with_capacity(usize_2);
    let mut vec_1: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut bytesmut_4: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from_iter::<std::vec::Vec<u8>>(vec_1);
    let mut bytesmut_4_ref_0: &mut crate::bytes_mut::BytesMut = &mut bytesmut_4;
    let mut vec_2: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut bytesmut_5: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from_iter::<std::vec::Vec<u8>>(vec_2);
    let mut bytesmut_5_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_5;
    let mut vec_3: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut bytesmut_6: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from_iter::<std::vec::Vec<u8>>(vec_3);
    let mut bytesmut_6_ref_0: &crate::bytes_mut::BytesMut = &mut bytesmut_6;
    let mut usize_3: usize = 3230usize;
    let mut vec_4: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut bytesmut_7: crate::bytes_mut::BytesMut = crate::bytes_mut::BytesMut::from_iter::<std::vec::Vec<u8>>(vec_4);
    let mut bytes_0: crate::bytes::Bytes = crate::bytes_mut::BytesMut::freeze(bytesmut_7);
    let mut bytes_0_ref_0: &mut crate::bytes::Bytes = &mut bytes_0;
    let mut usize_4: usize = 7004usize;
    let mut bytes_1: crate::bytes::Bytes = crate::bytes::Bytes::copy_to_bytes(bytes_0_ref_0, usize_3);
    let mut option_0: std::option::Option<std::cmp::Ordering> = crate::bytes_mut::BytesMut::partial_cmp(bytesmut_6_ref_0, bytesmut_5_ref_0);
    crate::bytes_mut::BytesMut::unsplit(bytesmut_4_ref_0, bytesmut_3);
    crate::bytes_mut::BytesMut::reserve(bytesmut_2_ref_0, usize_1);
    crate::bytes_mut::BytesMut::unsplit(bytesmut_1_ref_0, bytesmut_0);
    panic!("From RustyUnit with love");
}
}