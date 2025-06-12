// Answer 0

#[test]
fn test_join_spans_empty() {
    use quote::ToTokens;
    use proc_macro2::{TokenStream, Span};

    // Creating an empty TokenStream to test the edge case where iter.next() is None.
    let empty_tokens: TokenStream = TokenStream::new();

    // Calling the join_spans function with an empty TokenStream.
    let result = join_spans(empty_tokens);

    // The expected result should be the Span::call_site() since there are no spans to join.
    assert_eq!(result, Span::call_site());
}

#[test]
fn test_join_spans_single_token() {
    use quote::quote;
    use proc_macro2::{TokenStream, Span};

    // Creating a TokenStream with a single token to check behavior with one element.
    let single_token: TokenStream = quote! { /* single token */ };

    // Calling the join_spans function with the single token.
    let result = join_spans(single_token);

    // The expected result should be equal to the span of the single token as there are no other tokens to join.
    assert_eq!(result, single_token.span());
}

#[test]
fn test_join_spans_multiple_tokens() {
    use quote::quote;
    use proc_macro2::{TokenStream, Span};

    // Creating a TokenStream with multiple tokens to check the joining behavior.
    let multiple_tokens: TokenStream = quote! { token1 token2 token3 };

    // Calling the join_spans function with multiple tokens.
    let result = join_spans(multiple_tokens);

    // The expected result should be the join of the spans of the first and last tokens.
    let first_span = quote! { token1 }.span();
    let last_span = quote! { token3 }.span();
    let expected_span = first_span.join(last_span).unwrap_or(first_span);
    
    assert_eq!(result, expected_span);
}

