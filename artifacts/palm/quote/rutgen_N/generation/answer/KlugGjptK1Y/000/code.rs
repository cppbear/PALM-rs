// Answer 0

#[test]
fn test_append_terminated() {
    struct MockTokens {
        output: String,
    }

    impl MockTokens {
        fn new() -> Self {
            MockTokens { output: String::new() }
        }

        fn to_tokens(&mut self, token: &str) {
            self.output.push_str(token);
        }
    }

    impl ToTokens for &str {
        fn to_tokens(self, tokens: &mut dyn ToTokens) {
            tokens.to_tokens(self);
        }
    }

    let mut mock_tokens = MockTokens::new();
    let items = vec!["item1", "item2"];
    let terminator = ";";

    append_terminated(&mut mock_tokens, items.iter().cloned(), terminator);

    assert_eq!(mock_tokens.output, "item1;item2;");
}

#[test]
fn test_append_terminated_empty() {
    struct MockTokens {
        output: String,
    }

    impl MockTokens {
        fn new() -> Self {
            MockTokens { output: String::new() }
        }

        fn to_tokens(&mut self, token: &str) {
            self.output.push_str(token);
        }
    }

    impl ToTokens for &str {
        fn to_tokens(self, tokens: &mut dyn ToTokens) {
            tokens.to_tokens(self);
        }
    }

    let mut mock_tokens = MockTokens::new();
    let items: Vec<&str> = Vec::new();
    let terminator = ";";

    append_terminated(&mut mock_tokens, items.into_iter(), terminator);

    assert_eq!(mock_tokens.output, "");
}

