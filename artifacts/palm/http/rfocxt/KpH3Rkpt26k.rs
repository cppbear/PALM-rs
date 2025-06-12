use std::convert::TryInto;
use super::{Authority, Parts, PathAndQuery, Scheme};
use crate::Uri;
#[derive(Debug)]
pub struct Builder {
    parts: Result<Parts, crate::Error>,
}
#[derive(Clone)]
pub struct Uri {
    scheme: Scheme,
    authority: Authority,
    path_and_query: PathAndQuery,
}
#[derive(Debug)]
pub struct InvalidUriParts(InvalidUri);
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
pub struct Error {
    inner: ErrorKind,
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
    pub fn build(self) -> Result<Uri, crate::Error> {
        let parts = self.parts?;
        Uri::from_parts(parts).map_err(Into::into)
    }
    fn map<F>(self, func: F) -> Self
    where
        F: FnOnce(Parts) -> Result<Parts, crate::Error>,
    {
        Builder {
            parts: self.parts.and_then(func),
        }
    }
}
impl Uri {
    pub fn builder() -> Builder {}
    pub fn from_parts(src: Parts) -> Result<Uri, InvalidUriParts> {
        if src.scheme.is_some() {
            if src.authority.is_none() {
                return Err(ErrorKind::AuthorityMissing.into());
            }
            if src.path_and_query.is_none() {
                return Err(ErrorKind::PathAndQueryMissing.into());
            }
        } else if src.authority.is_some() && src.path_and_query.is_some() {
            return Err(ErrorKind::SchemeMissing.into());
        }
        let scheme = match src.scheme {
            Some(scheme) => scheme,
            None => Scheme { inner: Scheme2::None },
        };
        let authority = match src.authority {
            Some(authority) => authority,
            None => Authority::empty(),
        };
        let path_and_query = match src.path_and_query {
            Some(path_and_query) => path_and_query,
            None => PathAndQuery::empty(),
        };
        Ok(Uri {
            scheme,
            authority,
            path_and_query,
        })
    }
    pub fn from_maybe_shared<T>(src: T) -> Result<Self, InvalidUri>
    where
        T: AsRef<[u8]> + 'static,
    {
        if_downcast_into!(T, Bytes, src, { return Uri::from_shared(src); });
        Uri::try_from(src.as_ref())
    }
    fn from_shared(s: Bytes) -> Result<Uri, InvalidUri> {}
    pub fn from_static(src: &'static str) -> Self {
        let s = Bytes::from_static(src.as_bytes());
        match Uri::from_shared(s) {
            Ok(uri) => uri,
            Err(e) => panic!("static str is not valid URI: {}", e),
        }
    }
    #[inline]
    pub fn into_parts(self) -> Parts {}
    #[inline]
    pub fn path_and_query(&self) -> Option<&PathAndQuery> {}
    #[inline]
    pub fn path(&self) -> &str {}
    #[inline]
    pub fn scheme(&self) -> Option<&Scheme> {}
    #[inline]
    pub fn scheme_str(&self) -> Option<&str> {}
    #[inline]
    pub fn authority(&self) -> Option<&Authority> {}
    #[inline]
    pub fn host(&self) -> Option<&str> {}
    pub fn port(&self) -> Option<Port<&str>> {}
    pub fn port_u16(&self) -> Option<u16> {}
    #[inline]
    pub fn query(&self) -> Option<&str> {}
    fn has_path(&self) -> bool {}
}
