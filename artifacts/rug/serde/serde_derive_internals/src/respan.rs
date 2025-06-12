use proc_macro2::{Group, Span, TokenStream, TokenTree};

pub(crate) fn respan(stream: TokenStream, span: Span) -> TokenStream {
    stream
        .into_iter()
        .map(|token| respan_token(token, span))
        .collect()
}

fn respan_token(mut token: TokenTree, span: Span) -> TokenTree {
    if let TokenTree::Group(g) = &mut token {
        *g = Group::new(g.delimiter(), respan(g.stream(), span));
    }
    token.set_span(span);
    token
}

#[cfg(test)]
mod tests_llm_16_152 {
    use super::*;

use crate::*;
    use proc_macro2::{TokenStream, Span};
    use quote::quote;

    #[test]
    fn test_respan() {
        let original_tokens: TokenStream = quote! {
            let x = 42;
        };

        let new_span = Span::call_site(); // Use a new span for testing
        let respanned_tokens = respan(original_tokens.clone(), new_span);

        // Assert that the respanned tokens are the same count as original
        assert_eq!(respanned_tokens.clone().into_iter().count(), original_tokens.into_iter().count());

        // Further assertions can be added here to inspect the respanned tokens
    }
}
