use bytes::{Bytes, BytesMut};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::Write;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use std::{cmp, fmt, str};
use crate::header::name::HeaderName;
macro_rules! from_integers {
    ($($name:ident : $t:ident => $max_len:expr),*) => {
        $(impl From <$t > for HeaderValue { fn from(num : $t) -> HeaderValue { let mut
        buf = BytesMut::with_capacity($max_len); let _ = buf
        .write_str(::itoa::Buffer::new().format(num)); HeaderValue { inner : buf
        .freeze(), is_sensitive : false, } } })*
    };
}
from_integers! {
    from_u16 : u16 => 5, from_i16 : i16 => 6, from_u32 : u32 => 10, from_i32 : i32 => 11,
    from_u64 : u64 => 20, from_i64 : i64 => 20
}
#[cfg(target_pointer_width = "16")]
from_integers! {
    from_usize : usize => 5, from_isize : isize => 6
}
#[cfg(target_pointer_width = "32")]
from_integers! {
    from_usize : usize => 10, from_isize : isize => 11
}
#[cfg(target_pointer_width = "64")]
from_integers! {
    from_usize : usize => 20, from_isize : isize => 20
}
#[derive(Clone)]
pub struct HeaderValue {
    inner: Bytes,
    is_sensitive: bool,
}
pub struct InvalidHeaderValue {
    _priv: (),
}
impl TryFrom<Vec<u8>> for HeaderValue {
    type Error = InvalidHeaderValue;
    #[inline]
    fn try_from(vec: Vec<u8>) -> Result<Self, Self::Error> {
        HeaderValue::from_shared(vec.into())
    }
}
