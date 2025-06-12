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
impl hash::Hash for PathAndQuery {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}
