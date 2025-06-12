use self::extension::{AllocatedExtension, InlineExtension};
use self::Inner::*;
use std::convert::TryFrom;
use std::error::Error;
use std::str::FromStr;
use std::{fmt, str};
pub struct InvalidMethod {
    _priv: (),
}
impl fmt::Debug for InvalidMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("InvalidMethod").finish()
    }
}
