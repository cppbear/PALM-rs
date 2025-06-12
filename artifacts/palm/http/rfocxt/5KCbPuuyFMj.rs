use self::extension::{AllocatedExtension, InlineExtension};
use self::Inner::*;
use std::convert::TryFrom;
use std::error::Error;
use std::str::FromStr;
use std::{fmt, str};
pub struct InvalidMethod {
    _priv: (),
}
impl fmt::Display for InvalidMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid HTTP method")
    }
}
