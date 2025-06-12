// Answer 0

#[test]
fn test_to_tokens_true() {
    let mut tokens = TokenStream::new();
    let value: &bool = &true;
    value.to_tokens(&mut tokens);
}

#[test]
fn test_to_tokens_false() {
    let mut tokens = TokenStream::new();
    let value: &bool = &false;
    value.to_tokens(&mut tokens);
}

