use self::extension::{AllocatedExtension, InlineExtension};
use self::Inner::*;
use std::convert::TryFrom;
use std::error::Error;
use std::str::FromStr;
use std::{fmt, str};
pub struct InvalidMethod {
    _priv: (),
}
impl InvalidMethod {
    fn new() -> InvalidMethod {
        InvalidMethod { _priv: () }
    }
}
