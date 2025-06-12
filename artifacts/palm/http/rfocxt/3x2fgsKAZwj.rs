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
impl TryFrom<String> for Authority {
    type Error = InvalidUri;
    #[inline]
    fn try_from(t: String) -> Result<Self, Self::Error> {
        Authority::from_shared(t.into())
    }
}
