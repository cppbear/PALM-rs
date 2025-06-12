pub type Result<T> = result::Result<T, Error>;
use std::error;
use std::fmt;
use std::result;
use crate::header;
use crate::header::MaxSizeReached;
use crate::method;
use crate::status;
use crate::uri;
pub struct Error {
    inner: ErrorKind,
}
#[derive(Debug, Eq, PartialEq)]
enum ErrorKind {
    InvalidUriChar,
    InvalidScheme,
    InvalidAuthority,
    InvalidPort,
    InvalidFormat,
    SchemeMissing,
    AuthorityMissing,
    PathAndQueryMissing,
    TooLong,
    Empty,
    SchemeTooLong,
}
enum ErrorKind {
    StatusCode(status::InvalidStatusCode),
    Method(method::InvalidMethod),
    Uri(uri::InvalidUri),
    UriParts(uri::InvalidUriParts),
    HeaderName(header::InvalidHeaderName),
    HeaderValue(header::InvalidHeaderValue),
    MaxSizeReached(MaxSizeReached),
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.get_ref(), f)
    }
}
impl Error {
    pub fn is<T: error::Error + 'static>(&self) -> bool {}
    pub fn get_ref(&self) -> &(dyn error::Error + 'static) {
        use self::ErrorKind::*;
        match self.inner {
            StatusCode(ref e) => e,
            Method(ref e) => e,
            Uri(ref e) => e,
            UriParts(ref e) => e,
            HeaderName(ref e) => e,
            HeaderValue(ref e) => e,
            MaxSizeReached(ref e) => e,
        }
    }
}
