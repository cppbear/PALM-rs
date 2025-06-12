// Answer 0

#[test]
fn test_join_spans_with_multiple_spans() {
    use quote::TokenStream;
    use proc_macro2::{Span, TokenTree};

    let tokens = TokenStream::from_iter(vec![
        TokenTree::Group(proc_macro2::Group::new(Span::new(1, 2), TokenStream::new())),
        TokenTree::Group(proc_macro2::Group::new(Span::new(3, 4), TokenStream::new())),
        TokenTree::Group(proc_macro2::Group::new(Span::new(5, 6), TokenStream::new())),
    ]);

    let result = join_spans(tokens);
    assert_ne!(result, Span::call_site());
}

#[test]
fn test_join_spans_with_two_spans() {
    use quote::TokenStream;
    use proc_macro2::{Span, TokenTree};

    let tokens = TokenStream::from_iter(vec![
        TokenTree::Group(proc_macro2::Group::new(Span::new(10, 20), TokenStream::new())),
        TokenTree::Group(proc_macro2::Group::new(Span::new(30, 40), TokenStream::new())),
    ]);

    let result = join_spans(tokens);
    assert_ne!(result, Span::call_site());
}

#[test]
fn test_join_spans_with_no_spans() {
    use quote::TokenStream;

    let tokens = TokenStream::new(); // No tokens

    let result = join_spans(tokens);
    assert_eq!(result, Span::call_site());
}

#[test]
fn test_join_spans_with_one_span() {
    use quote::TokenStream;
    use proc_macro2::{Span, TokenTree};

    let tokens = TokenStream::from_iter(vec![
        TokenTree::Group(proc_macro2::Group::new(Span::new(100, 200), TokenStream::new())),
    ]);

    let result = join_spans(tokens);
    assert_eq!(result, Span::new(100, 200)); // Should return the span of the single token
}

