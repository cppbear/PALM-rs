// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    use proc_macro2::TokenStream;
    use proc_macro2::Literal;

    struct TestString<'a>(&'a str);

    impl<'a> ToTokens for TestString<'a> {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::string(self.0));
        }
    }

    #[test]
    fn test_to_tokens() {
        let test_str = TestString("Hello, world!");
        let mut tokens = TokenStream::new();
        test_str.to_tokens(&mut tokens);
        
        assert_eq!(tokens.to_string(), r#""Hello, world!""#);
    }

    #[test]
    fn test_to_tokens_empty_string() {
        let test_str = TestString("");
        let mut tokens = TokenStream::new();
        test_str.to_tokens(&mut tokens);
        
        assert_eq!(tokens.to_string(), r#""""#);
    }
}

