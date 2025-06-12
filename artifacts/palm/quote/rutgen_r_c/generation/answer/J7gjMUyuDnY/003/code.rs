// Answer 0

#[test]
fn test_append_separated_with_empty_iter() {
    let mut token_stream = TokenStream::new();
    let separator = MockToken { value: "sep" };

    let tokens: Vec<MockToken> = vec![];
    token_stream.append_separated(tokens, separator);

    // Check that the TokenStream is still empty
    assert!(token_stream.is_empty());
}

#[test]
fn test_append_separated_with_one_element() {
    let mut token_stream = TokenStream::new();
    let separator = MockToken { value: "sep" };
    let token = MockToken { value: "token1" };

    let tokens: Vec<MockToken> = vec![token];
    token_stream.append_separated(tokens, separator);

    // Check that the TokenStream contains only the single token
    assert!(token_stream.to_string() == "token1");
}

#[test]
fn test_append_separated_with_two_elements() {
    let mut token_stream = TokenStream::new();
    let separator = MockToken { value: "sep" };
    let token1 = MockToken { value: "token1" };
    let token2 = MockToken { value: "token2" };

    let tokens: Vec<MockToken> = vec![token1, token2];
    token_stream.append_separated(tokens, separator);

    // Check that the TokenStream contains both tokens separated by the separator
    assert!(token_stream.to_string() == "token1sep token2");
}

#[test]
fn test_append_separated_with_multiple_elements() {
    let mut token_stream = TokenStream::new();
    let separator = MockToken { value: ", " };
    let token1 = MockToken { value: "token1" };
    let token2 = MockToken { value: "token2" };
    let token3 = MockToken { value: "token3" };

    let tokens: Vec<MockToken> = vec![token1, token2, token3];
    token_stream.append_separated(tokens, separator);

    // Check that the TokenStream contains all tokens separated by the separator
    assert!(token_stream.to_string() == "token1, token2 token3");
}

struct MockToken {
    value: &'static str,
}

impl ToTokens for MockToken {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(quote::quote! { #self.value });
    }

    fn to_token_stream(&self) -> TokenStream {
        self.into_token_stream()
    }

    fn into_token_stream(self) -> TokenStream {
        let mut ts = TokenStream::new();
        ts.extend(quote::quote! { #self.value });
        ts
    }
}

