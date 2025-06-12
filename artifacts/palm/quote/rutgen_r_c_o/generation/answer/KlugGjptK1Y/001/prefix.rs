// Answer 0

#[test]
fn test_append_terminated_with_true_tokens() {
    struct TrueToken;
    impl ToTokens for TrueToken {
        fn to_tokens(&self, _tokens: &mut TokenStream) {}
        fn to_token_stream(&self) -> TokenStream { TokenStream::new() }
        fn into_token_stream(self) -> TokenStream { TokenStream::new() }
    }

    let mut ts = TokenStream::new();
    let tokens = vec![TrueToken; 10];
    let terminator = TrueToken;
    ts.append_terminated(tokens, terminator);
}

#[test]
fn test_append_terminated_with_false_tokens() {
    struct FalseToken;
    impl ToTokens for FalseToken {
        fn to_tokens(&self, _tokens: &mut TokenStream) {}
        fn to_token_stream(&self) -> TokenStream { TokenStream::new() }
        fn into_token_stream(self) -> TokenStream { TokenStream::new() }
    }

    let mut ts = TokenStream::new();
    let tokens = vec![FalseToken; 10];
    let terminator = FalseToken;
    ts.append_terminated(tokens, terminator);
}

#[test]
fn test_append_terminated_with_mixed_tokens() {
    struct MixedToken;
    impl ToTokens for MixedToken {
        fn to_tokens(&self, _tokens: &mut TokenStream) {}
        fn to_token_stream(&self) -> TokenStream { TokenStream::new() }
        fn into_token_stream(self) -> TokenStream { TokenStream::new() }
    }

    let mut ts = TokenStream::new();
    let tokens = vec![MixedToken; 5];
    let terminator = MixedToken;
    ts.append_terminated(tokens, terminator);
}

#[test]
#[should_panic]
fn test_append_terminated_with_empty_iterator() {
    let mut ts = TokenStream::new();
    let tokens: Vec<()>;
    let terminator = TrueToken;
    ts.append_terminated(tokens, terminator);
}

