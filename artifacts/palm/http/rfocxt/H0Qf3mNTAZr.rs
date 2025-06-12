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
    pub fn host(&self) -> &str {}
    pub fn port(&self) -> Option<Port<&str>> {}
    pub fn port_u16(&self) -> Option<u16> {}
    #[inline]
    pub fn as_str(&self) -> &str {}
}
fn create_authority<B, F>(b: B, f: F) -> Result<Authority, InvalidUri>
where
    B: AsRef<[u8]>,
    F: FnOnce(B) -> Bytes,
{
    let s = b.as_ref();
    let authority_end = Authority::parse_non_empty(s)?;
    if authority_end != s.len() {
        return Err(ErrorKind::InvalidUriChar.into());
    }
    let bytes = f(b);
    Ok(Authority {
        data: unsafe { ByteStr::from_utf8_unchecked(bytes) },
    })
}
