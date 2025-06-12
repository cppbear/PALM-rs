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
impl ToTokens for Literal {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(self.clone());
    }
}
