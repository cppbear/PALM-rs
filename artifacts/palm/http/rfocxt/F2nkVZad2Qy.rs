use std::fmt;
use super::{ErrorKind, InvalidUri};
pub struct Port<T> {
    port: u16,
    repr: T,
}
#[derive(Debug)]
pub struct InvalidUri(ErrorKind);
impl<T> Port<T>
where
    T: AsRef<str>,
{
    pub(crate) fn from_str(bytes: T) -> Result<Self, InvalidUri> {
        bytes
            .as_ref()
            .parse::<u16>()
            .map(|port| Port { port, repr: bytes })
            .map_err(|_| ErrorKind::InvalidPort.into())
    }
    pub fn as_str(&self) -> &str {}
}
