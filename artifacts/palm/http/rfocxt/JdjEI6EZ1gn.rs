use std::fmt;
use super::{ErrorKind, InvalidUri};
pub struct Port<T> {
    port: u16,
    repr: T,
}
impl<T> Port<T> {
    pub const fn as_u16(&self) -> u16 {
        self.port
    }
}
