// Answer 0

#[test]
fn test_append_terminated_with_empty_iterator() {
    let mut tokens = TokenStream::new();
    let term = DummyToken;
    let iter: Vec<DummyToken> = vec![];

    tokens.append_terminated(iter, term);

    // Verify that the tokens stream remains empty
    assert!(tokens.is_empty());
}

#[test]
fn test_append_terminated_with_one_element_iterator() {
    let mut tokens = TokenStream::new();
    let term = DummyToken;
    let iter = vec![DummyToken];

    tokens.append_terminated(iter, term);

    // Verify that tokens contain the token and the terminator
    assert_eq!(tokens.to_string(), "DummyTokenDummyToken");
}

#[test]
fn test_append_terminated_with_multiple_elements_iterator() {
    let mut tokens = TokenStream::new();
    let term = DummyToken;
    let iter = vec![DummyToken, DummyToken];

    tokens.append_terminated(iter, term);

    // Verify that tokens contain all tokens and terminators
    assert_eq!(tokens.to_string(), "DummyTokenDummyTokenDummyTokenDummyToken");
}

#[should_panic]
fn test_append_terminated_with_invalid_token() {
    struct InvalidToken;

    impl ToTokens for InvalidToken {
        fn to_tokens(&self, _tokens: &mut TokenStream) {
            panic!("Invalid token panic");
        }
        fn to_token_stream(&self) -> TokenStream {
            TokenStream::new()
        }
        fn into_token_stream(self) -> TokenStream {
            TokenStream::new()
        }
    }

    let mut tokens = TokenStream::new();
    let term = DummyToken;
    let iter = vec![InvalidToken];

    tokens.append_terminated(iter, term);
}

struct DummyToken;

impl ToTokens for DummyToken {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(quote::quote! { DummyToken });
    }
    fn to_token_stream(&self) -> TokenStream {
        TokenStream::new()
    }
    fn into_token_stream(self) -> TokenStream {
        TokenStream::new()
    }
}

