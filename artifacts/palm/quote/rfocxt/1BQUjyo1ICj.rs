use super::ToTokens;
use core::iter;
use proc_macro2::{TokenStream, TokenTree};
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
pub trait ToTokens {
    fn to_tokens(&self, tokens: &mut TokenStream);
    fn to_token_stream(&self) -> TokenStream;
    fn into_token_stream(self) -> TokenStream
    where
        Self: Sized;
}
pub trait Sealed {}
impl TokenStreamExt for TokenStream {
    fn append<U>(&mut self, token: U)
    where
        U: Into<TokenTree>,
    {}
    fn append_all<I>(&mut self, iter: I)
    where
        I: IntoIterator,
        I::Item: ToTokens,
    {}
    fn append_separated<I, U>(&mut self, iter: I, op: U)
    where
        I: IntoIterator,
        I::Item: ToTokens,
        U: ToTokens,
    {
        for (i, token) in iter.into_iter().enumerate() {
            if i > 0 {
                op.to_tokens(self);
            }
            token.to_tokens(self);
        }
    }
    fn append_terminated<I, U>(&mut self, iter: I, term: U)
    where
        I: IntoIterator,
        I::Item: ToTokens,
        U: ToTokens,
    {}
}
