// Answer 0

#[test]
fn test_append_separated_empty_iterator() {
    let mut tokens = TokenStream::new();
    let empty_iter = vec![].into_iter();
    let separator = TokenTree::from(proc_macro2::Ident::new(",", proc_macro2::Span::call_site()));
    tokens.append_separated(empty_iter, separator);
}

#[test]
fn test_append_separated_one_item() {
    struct SingleToken;
    
    impl ToTokens for SingleToken {
        fn to_tokens(&self, tokens: &mut TokenStream) {}
    }
    
    let mut tokens = TokenStream::new();
    let single_iter = vec![SingleToken].into_iter();
    let separator = TokenTree::from(proc_macro2::Ident::new(",", proc_macro2::Span::call_site()));
    tokens.append_separated(single_iter, separator);
}

#[test]
fn test_append_separated_two_items() {
    struct TokenA;
    struct TokenB;
    
    impl ToTokens for TokenA {
        fn to_tokens(&self, tokens: &mut TokenStream) {}
    }
    
    impl ToTokens for TokenB {
        fn to_tokens(&self, tokens: &mut TokenStream) {}
    }
    
    let mut tokens = TokenStream::new();
    let tokens_iter = vec![TokenA, TokenB].into_iter();
    let separator = TokenTree::from(proc_macro2::Ident::new(",", proc_macro2::Span::call_site()));
    tokens.append_separated(tokens_iter, separator);
}

#[test]
fn test_append_separated_three_items() {
    struct TokenX;
    struct TokenY;
    struct TokenZ;
    
    impl ToTokens for TokenX {
        fn to_tokens(&self, tokens: &mut TokenStream) {}
    }
    
    impl ToTokens for TokenY {
        fn to_tokens(&self, tokens: &mut TokenStream) {}
    }
    
    impl ToTokens for TokenZ {
        fn to_tokens(&self, tokens: &mut TokenStream) {}
    }
    
    let mut tokens = TokenStream::new();
    let tokens_iter = vec![TokenX, TokenY, TokenZ].into_iter();
    let separator = TokenTree::from(proc_macro2::Ident::new(",", proc_macro2::Span::call_site()));
    tokens.append_separated(tokens_iter, separator);
}

