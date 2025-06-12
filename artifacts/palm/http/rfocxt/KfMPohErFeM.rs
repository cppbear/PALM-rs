use std::fmt;
use super::{ErrorKind, InvalidUri};
pub struct Port<T> {
    port: u16,
    repr: T,
}
impl<T> fmt::Debug for Port<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Port").field(&self.port).finish()
    }
}
