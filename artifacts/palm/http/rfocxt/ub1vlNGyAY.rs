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
#[derive(Debug)]
pub struct InvalidUri(ErrorKind);
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
impl Scheme2<usize> {
    fn parse_exact(s: &[u8]) -> Result<Scheme2<()>, InvalidUri> {
        match s {
            b"http" => Ok(Protocol::Http.into()),
            b"https" => Ok(Protocol::Https.into()),
            _ => {
                if s.len() > MAX_SCHEME_LEN {
                    return Err(ErrorKind::SchemeTooLong.into());
                }
                for &b in s {
                    match SCHEME_CHARS[b as usize] {
                        b':' => {
                            return Err(ErrorKind::InvalidScheme.into());
                        }
                        0 => {
                            return Err(ErrorKind::InvalidScheme.into());
                        }
                        _ => {}
                    }
                }
                Ok(Scheme2::Other(()))
            }
        }
    }
    pub(super) fn parse(s: &[u8]) -> Result<Scheme2<usize>, InvalidUri> {}
}
