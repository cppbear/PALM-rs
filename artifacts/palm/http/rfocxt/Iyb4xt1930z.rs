use std::any::Any;
use std::convert::TryInto;
use std::fmt;
use crate::header::{HeaderMap, HeaderName, HeaderValue};
use crate::status::StatusCode;
use crate::version::Version;
use crate::{Extensions, Result};
#[derive(Clone)]
pub struct Parts {
    /// The response's status
    pub status: StatusCode,
    /// The response's version
    pub version: Version,
    /// The response's headers
    pub headers: HeaderMap<HeaderValue>,
    /// The response's extensions
    pub extensions: Extensions,
    _priv: (),
}
#[derive(Clone)]
pub struct HeaderMap<T = HeaderValue> {
    mask: Size,
    indices: Box<[Pos]>,
    entries: Vec<Bucket<T>>,
    extra_values: Vec<ExtraValue<T>>,
    danger: Danger,
}
#[derive(PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash)]
pub struct Version(Http);
#[derive(Clone, Default)]
pub struct Extensions {
    map: Option<Box<AnyMap>>,
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StatusCode(NonZeroU16);
#[derive(Clone)]
pub struct HeaderValue {
    inner: Bytes,
    is_sensitive: bool,
}
impl fmt::Debug for Parts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Parts")
            .field("status", &self.status)
            .field("version", &self.version)
            .field("headers", &self.headers)
            .finish()
    }
}
