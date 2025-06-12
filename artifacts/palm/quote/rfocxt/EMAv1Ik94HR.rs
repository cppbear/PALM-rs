use crate::ToTokens;
use proc_macro2::extra::DelimSpan;
use proc_macro2::{Span, TokenStream};
pub trait Spanned: private::Sealed {
    fn __span(&self) -> Span;
}
pub trait Sealed {}
impl Spanned for DelimSpan {
    fn __span(&self) -> Span {
        self.join()
    }
}
