// Answer 0

#[test]
fn test_append_separated_empty_iterator() {
    let mut tokens = TokenStream::new();
    let iter: Vec<&str> = Vec::new();

    // Creating a struct to implement ToTokens for our test
    struct TestToken<'a>(&'a str);

    impl ToTokens for TestToken<'_> {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.extend(self.0.parse::<TokenStream>().unwrap());
        }
        fn to_token_stream(&self) -> TokenStream {
            let mut stream = TokenStream::new();
            self.to_tokens(&mut stream);
            stream
        }
        fn into_token_stream(self) -> TokenStream {
            self.to_token_stream()
        }
    }

    // No tokens should be appended, as the iterator is empty.
    tokens.append_separated(iter.iter().map(|&s| TestToken(s)), TestToken(","));
    assert!(tokens.is_empty());
}

#[test]
fn test_append_separated_single_item() {
    let mut tokens = TokenStream::new();
    let iter = vec!["token1"];

    struct TestToken<'a>(&'a str);

    impl ToTokens for TestToken<'_> {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.extend(self.0.parse::<TokenStream>().unwrap());
        }
        fn to_token_stream(&self) -> TokenStream {
            let mut stream = TokenStream::new();
            self.to_tokens(&mut stream);
            stream
        }
        fn into_token_stream(self) -> TokenStream {
            self.to_token_stream()
        }
    }

    // Only "token1" should be appended without any operator since i == 0
    tokens.append_separated(iter.iter().map(|&s| TestToken(s)), TestToken(","));
    assert_eq!(tokens.to_string(), "token1");
}

#[test]
fn test_append_separated_multiple_items() {
    let mut tokens = TokenStream::new();
    let iter = vec!["token1", "token2", "token3"];

    struct TestToken<'a>(&'a str);

    impl ToTokens for TestToken<'_> {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.extend(self.0.parse::<TokenStream>().unwrap());
        }
        fn to_token_stream(&self) -> TokenStream {
            let mut stream = TokenStream::new();
            self.to_tokens(&mut stream);
            stream
        }
        fn into_token_stream(self) -> TokenStream {
            self.to_token_stream()
        }
    }

    // Operator should be appended between tokens
    tokens.append_separated(iter.iter().map(|&s| TestToken(s)), TestToken(","));
    assert_eq!(tokens.to_string(), "token1,token2,token3");
}

