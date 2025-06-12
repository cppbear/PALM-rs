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
pub trait TokenStreamExt: private::Sealed {
    fn append<U>(&mut self, token: U)
    where
        U: Into<TokenTree>;
    fn append_all<I>(&mut self, iter: I)
    where
        I: IntoIterator,
        I::Item: ToTokens;
    fn append_separated<I, U>(&mut self, iter: I, op: U)
    where
        I: IntoIterator,
        I::Item: ToTokens,
        U: ToTokens;
    fn append_terminated<I, U>(&mut self, iter: I, term: U)
    where
        I: IntoIterator,
        I::Item: ToTokens,
        U: ToTokens;
}
impl ToTokens for TokenStream {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(iter::once(self.clone()));
    }
    fn into_token_stream(self) -> TokenStream {}
}
