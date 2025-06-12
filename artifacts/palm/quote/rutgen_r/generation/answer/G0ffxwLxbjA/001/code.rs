// Answer 0

#[derive(Clone)]
struct Dummy {
    value: i32,
}

impl Dummy {
    fn new(value: i32) -> Self {
        Dummy { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::TokenStream;

    #[test]
    fn test_to_tokens_with_positive_value() {
        let mut tokens = TokenStream::new();
        let dummy = Dummy::new(42);
        dummy.to_tokens(&mut tokens);
        assert_eq!(tokens.to_string(), "Dummy { value: 42 }");
    }

    #[test]
    fn test_to_tokens_with_zero_value() {
        let mut tokens = TokenStream::new();
        let dummy = Dummy::new(0);
        dummy.to_tokens(&mut tokens);
        assert_eq!(tokens.to_string(), "Dummy { value: 0 }");
    }

    #[test]
    fn test_to_tokens_with_negative_value() {
        let mut tokens = TokenStream::new();
        let dummy = Dummy::new(-42);
        dummy.to_tokens(&mut tokens);
        assert_eq!(tokens.to_string(), "Dummy { value: -42 }");
    }

    #[should_panic]
    #[test]
    fn test_to_tokens_with_large_value() {
        let mut tokens = TokenStream::new();
        let dummy = Dummy::new(i32::MAX);
        dummy.to_tokens(&mut tokens);
        // Intentionally causing a panic for large value handling
        assert!(tokens.is_empty());
    }
}

