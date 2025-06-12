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
impl HeaderValue {
    #[inline]
    #[allow(unconditional_panic)]
    pub const fn from_static(src: &'static str) -> HeaderValue {}
    #[inline]
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(src: &str) -> Result<HeaderValue, InvalidHeaderValue> {}
    #[inline]
    pub fn from_name(name: HeaderName) -> HeaderValue {}
    #[inline]
    pub fn from_bytes(src: &[u8]) -> Result<HeaderValue, InvalidHeaderValue> {}
    pub fn from_maybe_shared<T>(src: T) -> Result<HeaderValue, InvalidHeaderValue>
    where
        T: AsRef<[u8]> + 'static,
    {
        if_downcast_into!(T, Bytes, src, { return HeaderValue::from_shared(src); });
        HeaderValue::from_bytes(src.as_ref())
    }
    pub unsafe fn from_maybe_shared_unchecked<T>(src: T) -> HeaderValue
    where
        T: AsRef<[u8]> + 'static,
    {
        if cfg!(debug_assertions) {
            match HeaderValue::from_maybe_shared(src) {
                Ok(val) => val,
                Err(_err) => {
                    panic!(
                        "HeaderValue::from_maybe_shared_unchecked() with invalid bytes"
                    );
                }
            }
        } else {
            if_downcast_into!(
                T, Bytes, src, { return HeaderValue { inner : src, is_sensitive : false,
                }; }
            );
            let src = Bytes::copy_from_slice(src.as_ref());
            HeaderValue {
                inner: src,
                is_sensitive: false,
            }
        }
    }
    fn from_shared(src: Bytes) -> Result<HeaderValue, InvalidHeaderValue> {}
    fn try_from_generic<T: AsRef<[u8]>, F: FnOnce(T) -> Bytes>(
        src: T,
        into: F,
    ) -> Result<HeaderValue, InvalidHeaderValue> {}
    pub fn to_str(&self) -> Result<&str, ToStrError> {}
    #[inline]
    pub fn len(&self) -> usize {}
    #[inline]
    pub fn is_empty(&self) -> bool {}
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {}
    #[inline]
    pub fn set_sensitive(&mut self, val: bool) {}
    #[inline]
    pub fn is_sensitive(&self) -> bool {}
}
