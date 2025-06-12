use std::convert::TryInto;
use super::{Authority, Parts, PathAndQuery, Scheme};
use crate::Uri;
#[derive(Debug)]
pub struct Builder {
    parts: Result<Parts, crate::Error>,
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
impl Builder {
    #[inline]
    pub fn new() -> Builder {}
    pub fn scheme<T>(self, scheme: T) -> Self
    where
        T: TryInto<Scheme>,
        <T as TryInto<Scheme>>::Error: Into<crate::Error>,
    {
        self.map(move |mut parts| {
            let scheme = scheme.try_into().map_err(Into::into)?;
            parts.scheme = Some(scheme);
            Ok(parts)
        })
    }
    pub fn authority<T>(self, auth: T) -> Self
    where
        T: TryInto<Authority>,
        <T as TryInto<Authority>>::Error: Into<crate::Error>,
    {
        self.map(move |mut parts| {
            let auth = auth.try_into().map_err(Into::into)?;
            parts.authority = Some(auth);
            Ok(parts)
        })
    }
    pub fn path_and_query<T>(self, p_and_q: T) -> Self
    where
        T: TryInto<PathAndQuery>,
        <T as TryInto<PathAndQuery>>::Error: Into<crate::Error>,
    {
        self.map(move |mut parts| {
            let p_and_q = p_and_q.try_into().map_err(Into::into)?;
            parts.path_and_query = Some(p_and_q);
            Ok(parts)
        })
    }
    pub fn build(self) -> Result<Uri, crate::Error> {}
    fn map<F>(self, func: F) -> Self
    where
        F: FnOnce(Parts) -> Result<Parts, crate::Error>,
    {
        Builder {
            parts: self.parts.and_then(func),
        }
    }
}
