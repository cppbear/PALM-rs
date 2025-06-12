use std::any::Any;
use std::convert::TryInto;
use std::fmt;
use crate::header::{HeaderMap, HeaderName, HeaderValue};
use crate::method::Method;
use crate::version::Version;
use crate::{Extensions, Result, Uri};
#[derive(Clone)]
pub struct Request<T> {
    head: Parts,
    body: T,
}
#[derive(Debug)]
pub struct Builder {
    inner: Result<Parts>,
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
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Method(Inner);
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
impl Request<()> {
    #[inline]
    pub fn builder() -> Builder {}
    pub fn get<T>(uri: T) -> Builder
    where
        T: TryInto<Uri>,
        <T as TryInto<Uri>>::Error: Into<crate::Error>,
    {}
    pub fn put<T>(uri: T) -> Builder
    where
        T: TryInto<Uri>,
        <T as TryInto<Uri>>::Error: Into<crate::Error>,
    {}
    pub fn post<T>(uri: T) -> Builder
    where
        T: TryInto<Uri>,
        <T as TryInto<Uri>>::Error: Into<crate::Error>,
    {
        Builder::new().method(Method::POST).uri(uri)
    }
    pub fn delete<T>(uri: T) -> Builder
    where
        T: TryInto<Uri>,
        <T as TryInto<Uri>>::Error: Into<crate::Error>,
    {}
    pub fn options<T>(uri: T) -> Builder
    where
        T: TryInto<Uri>,
        <T as TryInto<Uri>>::Error: Into<crate::Error>,
    {}
    pub fn head<T>(uri: T) -> Builder
    where
        T: TryInto<Uri>,
        <T as TryInto<Uri>>::Error: Into<crate::Error>,
    {}
    pub fn connect<T>(uri: T) -> Builder
    where
        T: TryInto<Uri>,
        <T as TryInto<Uri>>::Error: Into<crate::Error>,
    {}
    pub fn patch<T>(uri: T) -> Builder
    where
        T: TryInto<Uri>,
        <T as TryInto<Uri>>::Error: Into<crate::Error>,
    {}
    pub fn trace<T>(uri: T) -> Builder
    where
        T: TryInto<Uri>,
        <T as TryInto<Uri>>::Error: Into<crate::Error>,
    {}
}
impl Builder {
    #[inline]
    pub fn new() -> Builder {
        Builder::default()
    }
    pub fn method<T>(self, method: T) -> Builder
    where
        T: TryInto<Method>,
        <T as TryInto<Method>>::Error: Into<crate::Error>,
    {
        self.and_then(move |mut head| {
            let method = method.try_into().map_err(Into::into)?;
            head.method = method;
            Ok(head)
        })
    }
    pub fn method_ref(&self) -> Option<&Method> {}
    pub fn uri<T>(self, uri: T) -> Builder
    where
        T: TryInto<Uri>,
        <T as TryInto<Uri>>::Error: Into<crate::Error>,
    {
        self.and_then(move |mut head| {
            head.uri = uri.try_into().map_err(Into::into)?;
            Ok(head)
        })
    }
    pub fn uri_ref(&self) -> Option<&Uri> {}
    pub fn version(self, version: Version) -> Builder {}
    pub fn version_ref(&self) -> Option<&Version> {}
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
