use std::convert::TryFrom;
use std::str::FromStr;
use std::{cmp, fmt, hash, str};
use bytes::Bytes;
use super::{ErrorKind, InvalidUri};
use crate::byte_str::ByteStr;
const NONE: u16 = u16::MAX;
#[derive(Clone)]
pub struct PathAndQuery {
    pub(super) data: ByteStr,
    pub(super) query: u16,
}
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(crate) struct ByteStr {
    bytes: Bytes,
}
#[derive(Debug)]
pub struct InvalidUri(ErrorKind);
#[derive(Debug, Eq, PartialEq)]
enum ErrorKind {
    InvalidUriChar,
    InvalidScheme,
    InvalidAuthority,
    InvalidPort,
    InvalidFormat,
    SchemeMissing,
    AuthorityMissing,
    PathAndQueryMissing,
    TooLong,
    Empty,
    SchemeTooLong,
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
    pub fn query(&self) -> Option<&str> {}
    #[inline]
    pub fn as_str(&self) -> &str {}
}
impl ByteStr {
    #[inline]
    pub fn new() -> ByteStr {}
    #[inline]
    pub const fn from_static(val: &'static str) -> ByteStr {}
    #[inline]
    pub unsafe fn from_utf8_unchecked(bytes: Bytes) -> ByteStr {
        if cfg!(debug_assertions) {
            match str::from_utf8(&bytes) {
                Ok(_) => {}
                Err(err) => {
                    panic!(
                        "ByteStr::from_utf8_unchecked() with invalid bytes; error = {}, bytes = {:?}",
                        err, bytes
                    )
                }
            }
        }
        ByteStr { bytes }
    }
    pub(crate) fn from_utf8(bytes: Bytes) -> Result<ByteStr, std::str::Utf8Error> {
        str::from_utf8(&bytes)?;
        Ok(ByteStr { bytes })
    }
}
