use std::any::Any;
use std::convert::TryInto;
use std::fmt;
use crate::header::{HeaderMap, HeaderName, HeaderValue};
use crate::method::Method;
use crate::version::Version;
use crate::{Extensions, Result, Uri};
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
#[derive(PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash)]
pub struct Version(Http);
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Method(Inner);
#[derive(Clone)]
pub struct Uri {
    scheme: Scheme,
    authority: Authority,
    path_and_query: PathAndQuery,
}
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
            method: Method::default(),
            uri: Uri::default(),
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
impl Default for Method {
    #[inline]
    fn default() -> Method {
        Method::GET
    }
}
impl Default for Uri {
    #[inline]
    fn default() -> Uri {
        Uri {
            scheme: Scheme::empty(),
            authority: Authority::empty(),
            path_and_query: PathAndQuery::slash(),
        }
    }
}
impl<T> Default for HeaderMap<T> {
    fn default() -> Self {
        HeaderMap::try_with_capacity(0).expect("zero capacity should never fail")
    }
}
