use super::TokenStreamExt;
use alloc::borrow::Cow;
use alloc::rc::Rc;
use core::iter;
use proc_macro2::{Group, Ident, Literal, Punct, Span, TokenStream, TokenTree};
use std::ffi::{CStr, CString};
pub trait ToTokens {
    fn to_tokens(&self, tokens: &mut TokenStream);
    fn to_token_stream(&self) -> TokenStream {
        let mut tokens = TokenStream::new();
        self.to_tokens(&mut tokens);
        tokens
    }
    fn into_token_stream(self) -> TokenStream
    where
        Self: Sized,
    {
        self.to_token_stream()
    }
}
pub trait IdentFragment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result;
    fn span(&self) -> Option<Span>;
}
pub trait Spanned: private::Sealed {
    fn __span(&self) -> Span;
}
pub trait RepAsIteratorExt<'q> {
    type Iter: Iterator;
    fn quote_into_iter(&'q self) -> (Self::Iter, HasIter);
}
impl ToTokens for bool {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let word = if *self { "true" } else { "false" };
        tokens.append(Ident::new(word, Span::call_site()));
    }
}
