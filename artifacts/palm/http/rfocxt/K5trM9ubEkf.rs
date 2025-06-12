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
#[derive(Clone)]
pub struct PathAndQuery {
    pub(super) data: ByteStr,
    pub(super) query: u16,
}
#[derive(Clone)]
pub struct Scheme {
    pub(super) inner: Scheme2,
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
    pub fn authority(&self) -> Option<&Authority> {}
    #[inline]
    pub fn host(&self) -> Option<&str> {}
    pub fn port(&self) -> Option<Port<&str>> {}
    pub fn port_u16(&self) -> Option<u16> {}
    #[inline]
    pub fn query(&self) -> Option<&str> {
        self.path_and_query.query()
    }
    fn has_path(&self) -> bool {}
}
impl PathAndQuery {
    pub(super) fn from_shared(mut src: Bytes) -> Result<Self, InvalidUri> {
        let mut query = NONE;
        let mut fragment = None;
        let mut is_maybe_not_utf8 = false;
        {
            let mut iter = src.as_ref().iter().enumerate();
            for (i, &b) in &mut iter {
                match b {
                    b'?' => {
                        debug_assert_eq!(query, NONE);
                        query = i as u16;
                        break;
                    }
                    b'#' => {
                        fragment = Some(i);
                        break;
                    }
                    #[rustfmt::skip]
                    0x21
                    | 0x24..=0x3B
                    | 0x3D
                    | 0x40..=0x5F
                    | 0x61..=0x7A
                    | 0x7C
                    | 0x7E => {}
                    0x7F..=0xFF => {
                        is_maybe_not_utf8 = true;
                    }
                    #[rustfmt::skip]
                    b'"' | b'{' | b'}' => {}
                    _ => return Err(ErrorKind::InvalidUriChar.into()),
                }
            }
            if query != NONE {
                for (i, &b) in iter {
                    match b {
                        #[rustfmt::skip]
                        0x21 | 0x24..=0x3B | 0x3D | 0x3F..=0x7E => {}
                        0x7F..=0xFF => {
                            is_maybe_not_utf8 = true;
                        }
                        b'#' => {
                            fragment = Some(i);
                            break;
                        }
                        _ => return Err(ErrorKind::InvalidUriChar.into()),
                    }
                }
            }
        }
        if let Some(i) = fragment {
            src.truncate(i);
        }
        let data = if is_maybe_not_utf8 {
            ByteStr::from_utf8(src).map_err(|_| ErrorKind::InvalidUriChar)?
        } else {
            unsafe { ByteStr::from_utf8_unchecked(src) }
        };
        Ok(PathAndQuery { data, query })
    }
    #[inline]
    pub fn from_static(src: &'static str) -> Self {
        let src = Bytes::from_static(src.as_bytes());
        PathAndQuery::from_shared(src).unwrap()
    }
    pub fn from_maybe_shared<T>(src: T) -> Result<Self, InvalidUri>
    where
        T: AsRef<[u8]> + 'static,
    {
        if_downcast_into!(T, Bytes, src, { return PathAndQuery::from_shared(src); });
        PathAndQuery::try_from(src.as_ref())
    }
    pub(super) fn empty() -> Self {
        PathAndQuery {
            data: ByteStr::new(),
            query: NONE,
        }
    }
    pub(super) fn slash() -> Self {
        PathAndQuery {
            data: ByteStr::from_static("/"),
            query: NONE,
        }
    }
    pub(super) fn star() -> Self {
        PathAndQuery {
            data: ByteStr::from_static("*"),
            query: NONE,
        }
    }
    #[inline]
    pub fn path(&self) -> &str {}
    #[inline]
    pub fn query(&self) -> Option<&str> {
        if self.query == NONE {
            None
        } else {
            let i = self.query + 1;
            Some(&self.data[i as usize..])
        }
    }
    #[inline]
    pub fn as_str(&self) -> &str {}
}
