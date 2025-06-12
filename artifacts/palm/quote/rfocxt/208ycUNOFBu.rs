use crate::ToTokens;
use proc_macro2::extra::DelimSpan;
use proc_macro2::{Span, TokenStream};
pub trait Spanned: private::Sealed {
    fn __span(&self) -> Span;
}
pub trait ToTokens {
    fn to_tokens(&self, tokens: &mut TokenStream);
    fn to_token_stream(&self) -> TokenStream;
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
pub trait RepAsIteratorExt<'q> {
    type Iter: Iterator;
    fn quote_into_iter(&'q self) -> (Self::Iter, HasIter);
}
pub trait Sealed {}
impl<T: ?Sized + ToTokens> Spanned for T {
    fn __span(&self) -> Span {
        join_spans(self.into_token_stream())
    }
}
fn join_spans(tokens: TokenStream) -> Span {
    let mut iter = tokens.into_iter().map(|tt| tt.span());
    let first = match iter.next() {
        Some(span) => span,
        None => return Span::call_site(),
    };
    iter.fold(None, |_prev, next| Some(next))
        .and_then(|last| first.join(last))
        .unwrap_or(first)
}
