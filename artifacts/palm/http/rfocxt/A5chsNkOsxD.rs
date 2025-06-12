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
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(crate) struct ByteStr {
    bytes: Bytes,
}
#[derive(Debug)]
pub struct InvalidUri(ErrorKind);
impl FromStr for Authority {
    type Err = InvalidUri;
    fn from_str(s: &str) -> Result<Self, InvalidUri> {
        TryFrom::try_from(s)
    }
}
