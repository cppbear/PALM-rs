use core::error::Error;
use core::panic::UnwindSafe;
pub trait AsDynError<'a>: Sealed {
    fn as_dyn_error(&self) -> &(dyn Error + 'a);
}
pub trait ThiserrorProvide: Sealed {
    fn thiserror_provide<'a>(&'a self, request: &mut Request<'a>);
}
pub trait AsDisplay<'a>: Sealed {
    type Target: Display;
    fn as_display(&'a self) -> Self::Target;
}
pub trait Sealed {}
impl<'a> AsDynError<'a> for dyn Error + Send + Sync + UnwindSafe + 'a {
    #[inline]
    fn as_dyn_error(&self) -> &(dyn Error + 'a) {
        self
    }
}
