use alloc::borrow::Cow;
use core::fmt;
use proc_macro2::{Ident, Span};
macro_rules! ident_fragment_display {
    ($($T:ty),*) => {
        $(impl IdentFragment for $T { fn fmt(& self, f : & mut fmt::Formatter) ->
        fmt::Result { fmt::Display::fmt(self, f) } })*
    };
}
ident_fragment_display!(bool, str, String, char);
ident_fragment_display!(u8, u16, u32, u64, u128, usize);
pub trait IdentFragment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result;
    fn span(&self) -> Option<Span> {
        None
    }
}
pub trait ToTokens {
    fn to_tokens(&self, tokens: &mut TokenStream);
    fn to_token_stream(&self) -> TokenStream;
    fn into_token_stream(self) -> TokenStream
    where
        Self: Sized;
}
impl<T> IdentFragment for Cow<'_, T>
where
    T: IdentFragment + ToOwned + ?Sized,
{
    fn span(&self) -> Option<Span> {}
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        T::fmt(self, f)
    }
}
