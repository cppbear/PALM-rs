// Answer 0

#[test]
fn test_to_tokens_with_punct() {
    use proc_macro2::{Punct, Spacing, TokenStream};

    let punct = Punct::new('!', Spacing::Alone);
    let mut tokens = TokenStream::new();
    punct.to_tokens(&mut tokens);
    
    let expected_tokens: TokenStream = TokenStream::from(punct.clone());
    assert_eq!(tokens.to_string(), expected_tokens.to_string());
}

#[test]
fn test_to_token_stream_with_punct() {
    use proc_macro2::{Punct, Spacing, TokenStream};

    let punct = Punct::new('?', Spacing::Alone);
    let tokens = punct.to_token_stream();
    
    let expected_tokens: TokenStream = TokenStream::from(punct.clone());
    assert_eq!(tokens.to_string(), expected_tokens.to_string());
}

#[test]
fn test_into_token_stream_with_punct() {
    use proc_macro2::{Punct, Spacing, TokenStream};

    let punct = Punct::new('.', Spacing::Alone);
    let tokens = punct.into_token_stream();
    
    let expected_tokens: TokenStream = TokenStream::from(punct.clone());
    assert_eq!(tokens.to_string(), expected_tokens.to_string());
}

