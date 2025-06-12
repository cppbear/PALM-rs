use std::convert::TryFrom;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use bytes::Bytes;
use super::{ErrorKind, InvalidUri};
use crate::byte_str::ByteStr;
const MAX_SCHEME_LEN: usize = 64;
#[rustfmt::skip]
const SCHEME_CHARS: [u8; 256] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, b'+', 0, b'-', b'.', 0, b'0', b'1',
    b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b':', 0, 0, 0, 0, 0, 0, b'A', b'B',
    b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
    b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', 0, 0, 0, 0, 0, 0, b'a',
    b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o',
    b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', 0, 0, 0, b'~', 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
#[derive(Clone)]
pub struct Scheme {
    pub(super) inner: Scheme2,
}
#[derive(Clone, Debug)]
pub(super) enum Scheme2<T = Box<ByteStr>> {
    None,
    Standard(Protocol),
    Other(T),
}
impl fmt::Display for Scheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
impl Scheme {
    pub const HTTP: Scheme = Scheme {
        inner: Scheme2::Standard(Protocol::Http),
    };
    pub const HTTPS: Scheme = Scheme {
        inner: Scheme2::Standard(Protocol::Https),
    };
    pub(super) fn empty() -> Self {
        Scheme { inner: Scheme2::None }
    }
    #[inline]
    pub fn as_str(&self) -> &str {
        use self::Protocol::*;
        use self::Scheme2::*;
        match self.inner {
            Standard(Http) => "http",
            Standard(Https) => "https",
            Other(ref v) => &v[..],
            None => unreachable!(),
        }
    }
}
