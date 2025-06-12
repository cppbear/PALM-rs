use std::convert::TryFrom;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use std::{cmp, fmt, str};
use bytes::Bytes;
use super::{ErrorKind, InvalidUri, Port, URI_CHARS};
use crate::byte_str::ByteStr;
#[derive(Clone)]
pub struct Authority {
    pub(super) data: ByteStr,
}
#[derive(Debug)]
pub struct InvalidUri(ErrorKind);
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(crate) struct ByteStr {
    bytes: Bytes,
}
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
impl Authority {
    pub(super) fn empty() -> Self {
        Authority { data: ByteStr::new() }
    }
    pub(super) fn from_shared(s: Bytes) -> Result<Self, InvalidUri> {
        create_authority(s, |s| s)
    }
    pub fn from_static(src: &'static str) -> Self {
        Authority::from_shared(Bytes::from_static(src.as_bytes()))
            .expect("static str is not valid authority")
    }
    pub fn from_maybe_shared<T>(src: T) -> Result<Self, InvalidUri>
    where
        T: AsRef<[u8]> + 'static,
    {
        if_downcast_into!(T, Bytes, src, { return Authority::from_shared(src); });
        Authority::try_from(src.as_ref())
    }
    pub(super) fn parse(s: &[u8]) -> Result<usize, InvalidUri> {
        let mut colon_cnt = 0u32;
        let mut start_bracket = false;
        let mut end_bracket = false;
        let mut has_percent = false;
        let mut end = s.len();
        let mut at_sign_pos = None;
        const MAX_COLONS: u32 = 8;
        for (i, &b) in s.iter().enumerate() {
            match URI_CHARS[b as usize] {
                b'/' | b'?' | b'#' => {
                    end = i;
                    break;
                }
                b':' => {
                    if colon_cnt >= MAX_COLONS {
                        return Err(ErrorKind::InvalidAuthority.into());
                    }
                    colon_cnt += 1;
                }
                b'[' => {
                    if has_percent || start_bracket {
                        return Err(ErrorKind::InvalidAuthority.into());
                    }
                    start_bracket = true;
                }
                b']' => {
                    if (!start_bracket) || end_bracket {
                        return Err(ErrorKind::InvalidAuthority.into());
                    }
                    end_bracket = true;
                    colon_cnt = 0;
                    has_percent = false;
                }
                b'@' => {
                    at_sign_pos = Some(i);
                    colon_cnt = 0;
                    has_percent = false;
                }
                0 if b == b'%' => {
                    has_percent = true;
                }
                0 => {
                    return Err(ErrorKind::InvalidUriChar.into());
                }
                _ => {}
            }
        }
        if start_bracket ^ end_bracket {
            return Err(ErrorKind::InvalidAuthority.into());
        }
        if colon_cnt > 1 {
            return Err(ErrorKind::InvalidAuthority.into());
        }
        if end > 0 && at_sign_pos == Some(end - 1) {
            return Err(ErrorKind::InvalidAuthority.into());
        }
        if has_percent {
            return Err(ErrorKind::InvalidAuthority.into());
        }
        Ok(end)
    }
    fn parse_non_empty(s: &[u8]) -> Result<usize, InvalidUri> {
        if s.is_empty() {
            return Err(ErrorKind::Empty.into());
        }
        Authority::parse(s)
    }
    #[inline]
    pub fn host(&self) -> &str {}
    pub fn port(&self) -> Option<Port<&str>> {}
    pub fn port_u16(&self) -> Option<u16> {}
    #[inline]
    pub fn as_str(&self) -> &str {}
}
