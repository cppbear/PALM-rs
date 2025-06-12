use core::error::{Error, Request};
pub trait ThiserrorProvide: Sealed {
    fn thiserror_provide<'a>(&'a self, request: &mut Request<'a>);
}
pub trait AsDisplay<'a>: Sealed {
    type Target: Display;
    fn as_display(&'a self) -> Self::Target;
}
pub trait AsDynError<'a>: Sealed {
    fn as_dyn_error(&self) -> &(dyn Error + 'a);
}
pub trait Sealed {}
impl<T> ThiserrorProvide for T
where
    T: Error + ?Sized,
{
    #[inline]
    fn thiserror_provide<'a>(&'a self, request: &mut Request<'a>) {
        self.provide(request);
    }
}
