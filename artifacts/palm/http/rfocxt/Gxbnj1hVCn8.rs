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
    pub(super) fn parse(s: &[u8]) -> Result<usize, InvalidUri> {}
    fn parse_non_empty(s: &[u8]) -> Result<usize, InvalidUri> {}
    #[inline]
    pub fn host(&self) -> &str {
        host(self.as_str())
    }
    pub fn port(&self) -> Option<Port<&str>> {}
    pub fn port_u16(&self) -> Option<u16> {}
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.data[..]
    }
}
fn host(auth: &str) -> &str {
    let host_port = auth.rsplit('@').next().expect("split always has at least 1 item");
    if host_port.as_bytes()[0] == b'[' {
        let i = host_port.find(']').expect("parsing should validate brackets");
        &host_port[0..i + 1]
    } else {
        host_port.split(':').next().expect("split always has at least 1 item")
    }
}
