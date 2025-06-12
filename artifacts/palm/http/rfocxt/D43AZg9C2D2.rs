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
pub struct InvalidStatusCode {
    _priv: (),
}
pub struct InvalidHeaderValue {
    _priv: (),
}
pub struct MaxSizeReached {
    _priv: (),
}
#[derive(Debug)]
pub struct InvalidUri(ErrorKind);
pub struct InvalidHeaderName {
    _priv: (),
}
#[derive(Debug)]
pub struct InvalidUriParts(InvalidUri);
pub struct InvalidMethod {
    _priv: (),
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
