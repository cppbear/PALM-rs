use core::fmt::Display;
#[cfg(feature = "std")]
use std::path::{self, Path, PathBuf};
pub trait AsDisplay<'a>: Sealed {
    type Target: Display;
    fn as_display(&'a self) -> Self::Target;
}
pub trait Sealed {}
#[cfg(feature = "std")]
impl<'a> AsDisplay<'a> for PathBuf {
    type Target = path::Display<'a>;
    #[inline]
    fn as_display(&'a self) -> Self::Target {
        self.display()
    }
}
