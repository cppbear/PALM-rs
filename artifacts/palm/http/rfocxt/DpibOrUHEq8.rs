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
pub struct PathAndQuery {
    pub(super) data: ByteStr,
    pub(super) query: u16,
}
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(crate) struct ByteStr {
    bytes: Bytes,
}
#[derive(Clone)]
pub struct Authority {
    pub(super) data: ByteStr,
}
#[derive(Debug)]
pub struct InvalidUri(ErrorKind);
#[derive(Clone)]
pub struct Scheme {
    pub(super) inner: Scheme2,
}
#[derive(Clone)]
pub struct Uri {
    scheme: Scheme,
    authority: Authority,
    path_and_query: PathAndQuery,
}
#[derive(Clone, Debug)]
pub(super) enum Scheme2<T = Box<ByteStr>> {
    None,
    Standard(Protocol),
    Other(T),
}
#[derive(Copy, Clone, Debug)]
pub(super) enum Protocol {
    Http,
    Https,
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
impl<T> Scheme2<T> {
    pub(super) fn is_none(&self) -> bool {
        matches!(* self, Scheme2::None)
    }
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
    pub(crate) fn from_utf8(bytes: Bytes) -> Result<ByteStr, std::str::Utf8Error> {}
}
impl Scheme2<usize> {
    fn parse_exact(s: &[u8]) -> Result<Scheme2<()>, InvalidUri> {}
    pub(super) fn parse(s: &[u8]) -> Result<Scheme2<usize>, InvalidUri> {
        if s.len() >= 7 {
            if s[..7].eq_ignore_ascii_case(b"http://") {
                return Ok(Protocol::Http.into());
            }
        }
        if s.len() >= 8 {
            if s[..8].eq_ignore_ascii_case(b"https://") {
                return Ok(Protocol::Https.into());
            }
        }
        if s.len() > 3 {
            for i in 0..s.len() {
                let b = s[i];
                match SCHEME_CHARS[b as usize] {
                    b':' => {
                        if s.len() < i + 3 {
                            break;
                        }
                        if &s[i + 1..i + 3] != b"//" {
                            break;
                        }
                        if i > MAX_SCHEME_LEN {
                            return Err(ErrorKind::SchemeTooLong.into());
                        }
                        return Ok(Scheme2::Other(i));
                    }
                    0 => break,
                    _ => {}
                }
            }
        }
        Ok(Scheme2::None)
    }
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
    fn parse_non_empty(s: &[u8]) -> Result<usize, InvalidUri> {}
    #[inline]
    pub fn host(&self) -> &str {}
    pub fn port(&self) -> Option<Port<&str>> {}
    pub fn port_u16(&self) -> Option<u16> {}
    #[inline]
    pub fn as_str(&self) -> &str {}
}
impl Protocol {
    pub(super) fn len(&self) -> usize {
        match *self {
            Protocol::Http => 4,
            Protocol::Https => 5,
        }
    }
}
fn parse_full(mut s: Bytes) -> Result<Uri, InvalidUri> {
    let scheme = match Scheme2::parse(&s[..])? {
        Scheme2::None => Scheme2::None,
        Scheme2::Standard(p) => {
            let _ = s.split_to(p.len() + 3);
            Scheme2::Standard(p)
        }
        Scheme2::Other(n) => {
            let mut scheme = s.split_to(n + 3);
            let _ = scheme.split_off(n);
            let val = unsafe { ByteStr::from_utf8_unchecked(scheme) };
            Scheme2::Other(Box::new(val))
        }
    };
    let authority_end = Authority::parse(&s[..])?;
    if scheme.is_none() {
        if authority_end != s.len() {
            return Err(ErrorKind::InvalidFormat.into());
        }
        let authority = Authority {
            data: unsafe { ByteStr::from_utf8_unchecked(s) },
        };
        return Ok(Uri {
            scheme: scheme.into(),
            authority,
            path_and_query: PathAndQuery::empty(),
        });
    }
    if authority_end == 0 {
        return Err(ErrorKind::InvalidFormat.into());
    }
    let authority = s.split_to(authority_end);
    let authority = Authority {
        data: unsafe { ByteStr::from_utf8_unchecked(authority) },
    };
    Ok(Uri {
        scheme: scheme.into(),
        authority,
        path_and_query: PathAndQuery::from_shared(s)?,
    })
}
