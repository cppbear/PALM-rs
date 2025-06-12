use std::any::Any;
use std::convert::TryInto;
use std::fmt;
use crate::header::{HeaderMap, HeaderName, HeaderValue};
use crate::status::StatusCode;
use crate::version::Version;
use crate::{Extensions, Result};
#[derive(Clone)]
pub struct Response<T> {
    head: Parts,
    body: T,
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
impl<T> Response<T> {
    #[inline]
    pub fn new(body: T) -> Response<T> {}
    #[inline]
    pub fn from_parts(parts: Parts, body: T) -> Response<T> {}
    #[inline]
    pub fn status(&self) -> StatusCode {}
    #[inline]
    pub fn status_mut(&mut self) -> &mut StatusCode {}
    #[inline]
    pub fn version(&self) -> Version {}
    #[inline]
    pub fn version_mut(&mut self) -> &mut Version {}
    #[inline]
    pub fn headers(&self) -> &HeaderMap<HeaderValue> {}
    #[inline]
    pub fn headers_mut(&mut self) -> &mut HeaderMap<HeaderValue> {}
    #[inline]
    pub fn extensions(&self) -> &Extensions {}
    #[inline]
    pub fn extensions_mut(&mut self) -> &mut Extensions {}
    #[inline]
    pub fn body(&self) -> &T {}
    #[inline]
    pub fn body_mut(&mut self) -> &mut T {}
    #[inline]
    pub fn into_body(self) -> T {
        self.body
    }
    #[inline]
    pub fn into_parts(self) -> (Parts, T) {}
    #[inline]
    pub fn map<F, U>(self, f: F) -> Response<U>
    where
        F: FnOnce(T) -> U,
    {}
}
