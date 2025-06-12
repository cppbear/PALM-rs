// Answer 0

#[test]
fn test_join_spans_empty() {
    use proc_macro2::{TokenStream, Span};

    let tokens = TokenStream::new();
    let result = join_spans(tokens);
    assert_eq!(result, Span::call_site());
}

#[test]
fn test_join_spans_single_token() {
    use proc_macro2::{TokenStream, Span, Ident};

    let tokens = TokenStream::from(Ident::new("foo", Span::new(1, 2)).into_token_stream());
    let result = join_spans(tokens);
    assert_eq!(result, Span::new(1, 2));
}

#[test]
fn test_join_spans_multiple_tokens() {
    use proc_macro2::{TokenStream, Span, Ident};

    let tokens = TokenStream::from(vec![
        Ident::new("foo", Span::new(1, 2)).into_token_stream(),
        Ident::new("bar", Span::new(3, 4)).into_token_stream(),
    ]);
    let result = join_spans(tokens);
    assert_eq!(result, Span::new(1, 4));
}

#[test]
fn test_join_spans_overlapping_tokens() {
    use proc_macro2::{TokenStream, Span, Ident};

    let tokens = TokenStream::from(vec![
        Ident::new("foo", Span::new(1, 3)).into_token_stream(),
        Ident::new("bar", Span::new(2, 4)).into_token_stream(),
    ]);
    let result = join_spans(tokens);
    assert_eq!(result, Span::new(1, 4));
}

