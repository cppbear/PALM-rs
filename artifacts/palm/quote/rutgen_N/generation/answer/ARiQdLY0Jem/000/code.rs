// Answer 0

#[test]
fn test_to_tokens() {
    use quote::ToTokens; // assuming there exists a trait `ToTokens`
    use proc_macro2::TokenStream;
    use proc_macro2::Literal;

    struct TestStruct {
        value: &'static str,
    }

    impl ToTokens for TestStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::string(self.value));
        }
    }

    let input = TestStruct { value: "Hello, world!" };
    let mut tokens = TokenStream::new();
    input.to_tokens(&mut tokens);

    let expected_tokens: TokenStream = Literal::string("Hello, world!").into();
    assert_eq!(tokens.to_string(), expected_tokens.to_string());
}

