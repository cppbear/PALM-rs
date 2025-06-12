use std::any::Any;
use std::convert::TryInto;
use std::fmt;
use crate::header::{HeaderMap, HeaderName, HeaderValue};
use crate::method::Method;
use crate::version::Version;
use crate::{Extensions, Result, Uri};
#[derive(Debug)]
pub struct Builder {
    inner: Result<Parts>,
}
#[derive(Clone)]
pub struct Parts {
    /// The request's method
    pub method: Method,
    /// The request's URI
    pub uri: Uri,
    /// The request's version
    pub version: Version,
    /// The request's headers
    pub headers: HeaderMap<HeaderValue>,
    /// The request's extensions
    pub extensions: Extensions,
    _priv: (),
}
pub struct Error {
    inner: ErrorKind,
}
#[derive(Debug, Default)]
pub struct Parts {
    /// The scheme component of a URI
    pub scheme: Option<Scheme>,
    /// The authority component of a URI
    pub authority: Option<Authority>,
    /// The origin-form component of a URI
    pub path_and_query: Option<PathAndQuery>,
    /// Allow extending in the future
    _priv: (),
}
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
impl Builder {
    #[inline]
    pub fn new() -> Builder {}
    pub fn method<T>(self, method: T) -> Builder
    where
        T: TryInto<Method>,
        <T as TryInto<Method>>::Error: Into<crate::Error>,
    {}
    pub fn method_ref(&self) -> Option<&Method> {}
    pub fn uri<T>(self, uri: T) -> Builder
    where
        T: TryInto<Uri>,
        <T as TryInto<Uri>>::Error: Into<crate::Error>,
    {}
    pub fn uri_ref(&self) -> Option<&Uri> {}
    pub fn version(self, version: Version) -> Builder {}
    pub fn version_ref(&self) -> Option<&Version> {
        self.inner.as_ref().ok().map(|h| &h.version)
    }
    pub fn header<K, V>(self, key: K, value: V) -> Builder
    where
        K: TryInto<HeaderName>,
        <K as TryInto<HeaderName>>::Error: Into<crate::Error>,
        V: TryInto<HeaderValue>,
        <V as TryInto<HeaderValue>>::Error: Into<crate::Error>,
    {}
    pub fn headers_ref(&self) -> Option<&HeaderMap<HeaderValue>> {}
    pub fn headers_mut(&mut self) -> Option<&mut HeaderMap<HeaderValue>> {}
    pub fn extension<T>(self, extension: T) -> Builder
    where
        T: Clone + Any + Send + Sync + 'static,
    {}
    pub fn extensions_ref(&self) -> Option<&Extensions> {}
    pub fn extensions_mut(&mut self) -> Option<&mut Extensions> {}
    pub fn body<T>(self, body: T) -> Result<Request<T>> {}
    fn and_then<F>(self, func: F) -> Self
    where
        F: FnOnce(Parts) -> Result<Parts>,
    {
        Builder {
            inner: self.inner.and_then(func),
        }
    }
}
