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
#[derive(PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash)]
pub struct Version(Http);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StatusCode(NonZeroU16);
#[derive(Clone)]
pub struct HeaderMap<T = HeaderValue> {
    mask: Size,
    indices: Box<[Pos]>,
    entries: Vec<Bucket<T>>,
    extra_values: Vec<ExtraValue<T>>,
    danger: Danger,
}
#[derive(Clone)]
pub struct HeaderValue {
    inner: Bytes,
    is_sensitive: bool,
}
#[derive(Clone, Default)]
pub struct Extensions {
    map: Option<Box<AnyMap>>,
}
impl Parts {
    fn new() -> Parts {
        Parts {
            status: StatusCode::default(),
            version: Version::default(),
            headers: HeaderMap::default(),
            extensions: Extensions::default(),
            _priv: (),
        }
    }
}
impl Default for Version {
    #[inline]
    fn default() -> Version {
        Version::HTTP_11
    }
}
impl Default for StatusCode {
    #[inline]
    fn default() -> StatusCode {
        StatusCode::OK
    }
}
impl<T> Default for HeaderMap<T> {
    fn default() -> Self {
        HeaderMap::try_with_capacity(0).expect("zero capacity should never fail")
    }
}
