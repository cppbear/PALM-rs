use std::fmt;
use super::{ErrorKind, InvalidUri};
pub struct Port<T> {
    port: u16,
    repr: T,
}
impl<T> fmt::Display for Port<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.port, f)
    }
}
