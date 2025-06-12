use crate::byte_str::ByteStr;
use std::convert::TryFrom;
use bytes::Bytes;
use std::error::Error;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::str::{self, FromStr};
use self::scheme::Scheme2;
pub use self::authority::Authority;
pub use self::builder::Builder;
pub use self::path::PathAndQuery;
pub use self::port::Port;
pub use self::scheme::Scheme;
const MAX_LEN: usize = (u16::MAX - 1) as usize;
#[rustfmt::skip]
const URI_CHARS: [u8; 256] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, b'!', 0, b'#', b'$', 0, b'&', b'\'', b'(', b')', b'*', b'+', b',',
    b'-', b'.', b'/', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b':',
    b';', 0, b'=', 0, b'?', b'@', b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I',
    b'J', b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W',
    b'X', b'Y', b'Z', b'[', 0, b']', 0, b'_', 0, b'a', b'b', b'c', b'd', b'e', b'f',
    b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't',
    b'u', b'v', b'w', b'x', b'y', b'z', 0, 0, 0, b'~', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0,
];
#[derive(Clone)]
pub struct Uri {
    scheme: Scheme,
    authority: Authority,
    path_and_query: PathAndQuery,
}
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(crate) struct ByteStr {
    bytes: Bytes,
}
#[derive(Clone)]
pub struct Scheme {
    pub(super) inner: Scheme2,
}
#[derive(Clone)]
pub struct PathAndQuery {
    pub(super) data: ByteStr,
    pub(super) query: u16,
}
#[derive(Clone)]
pub struct Authority {
    pub(super) data: ByteStr,
}
impl Uri {
    pub fn builder() -> Builder {}
    pub fn from_parts(src: Parts) -> Result<Uri, InvalidUriParts> {}
    pub fn from_maybe_shared<T>(src: T) -> Result<Self, InvalidUri>
    where
        T: AsRef<[u8]> + 'static,
    {
        if_downcast_into!(T, Bytes, src, { return Uri::from_shared(src); });
        Uri::try_from(src.as_ref())
    }
    fn from_shared(s: Bytes) -> Result<Uri, InvalidUri> {}
    pub fn from_static(src: &'static str) -> Self {
        let s = Bytes::from_static(src.as_bytes());
        match Uri::from_shared(s) {
            Ok(uri) => uri,
            Err(e) => panic!("static str is not valid URI: {}", e),
        }
    }
    #[inline]
    pub fn into_parts(self) -> Parts {}
    #[inline]
    pub fn path_and_query(&self) -> Option<&PathAndQuery> {}
    #[inline]
    pub fn path(&self) -> &str {}
    #[inline]
    pub fn scheme(&self) -> Option<&Scheme> {}
    #[inline]
    pub fn scheme_str(&self) -> Option<&str> {}
    #[inline]
    pub fn authority(&self) -> Option<&Authority> {
        if self.authority.data.is_empty() { None } else { Some(&self.authority) }
    }
    #[inline]
    pub fn host(&self) -> Option<&str> {}
    pub fn port(&self) -> Option<Port<&str>> {}
    pub fn port_u16(&self) -> Option<u16> {}
    #[inline]
    pub fn query(&self) -> Option<&str> {}
    fn has_path(&self) -> bool {}
}
