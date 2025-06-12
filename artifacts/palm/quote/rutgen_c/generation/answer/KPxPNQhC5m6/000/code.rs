// Answer 0

#[test]
fn test_to_tokens_bool_true() {
    use proc_macro2::{TokenStream, Span};
    
    struct BoolWrapper(bool);
    
    impl ToTokens for BoolWrapper {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.0.to_tokens(tokens);
        }
    }

    let value = BoolWrapper(true);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);

    let expected_tokens = TokenStream::from(Ident::new("true", Span::call_site()));
    assert_eq!(tokens.to_string(), expected_tokens.to_string());
}

#[test]
fn test_to_tokens_bool_false() {
    use proc_macro2::{TokenStream, Span};
    
    struct BoolWrapper(bool);
    
    impl ToTokens for BoolWrapper {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.0.to_tokens(tokens);
        }
    }

    let value = BoolWrapper(false);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);

    let expected_tokens = TokenStream::from(Ident::new("false", Span::call_site()));
    assert_eq!(tokens.to_string(), expected_tokens.to_string());
}

