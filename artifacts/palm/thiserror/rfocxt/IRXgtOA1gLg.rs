use core::fmt::Display;
#[cfg(feature = "std")]
use std::path::{self, Path, PathBuf};
pub trait AsDisplay<'a>: Sealed {
    type Target: Display;
    fn as_display(&'a self) -> Self::Target;
}
pub trait ThiserrorProvide: Sealed {
    fn thiserror_provide<'a>(&'a self, request: &mut Request<'a>);
}
pub trait AsDynError<'a>: Sealed {
    fn as_dyn_error(&self) -> &(dyn Error + 'a);
}
pub trait Sealed {}
impl<'a, T> AsDisplay<'a> for &T
where
    T: Display + ?Sized + 'a,
{
    type Target = &'a T;
    fn as_display(&'a self) -> Self::Target {
        *self
    }
}
