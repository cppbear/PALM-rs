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
impl fmt::Display for PathAndQuery {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.data.is_empty() {
            match self.data.as_bytes()[0] {
                b'/' | b'*' => write!(fmt, "{}", & self.data[..]),
                _ => write!(fmt, "/{}", & self.data[..]),
            }
        } else {
            write!(fmt, "/")
        }
    }
}
