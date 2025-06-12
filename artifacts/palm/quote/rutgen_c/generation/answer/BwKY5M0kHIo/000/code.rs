// Answer 0

#[test]
fn test_append_single_token() {
    use proc_macro2::{TokenStream, TokenTree, Group, Punct, Ident, Literal};
    
    let mut tokens = TokenStream::new();
    let token: TokenTree = Ident::new("test").into(); // example token

    tokens.append(token);
    assert_eq!(tokens.to_string(), "test");
}

#[test]
fn test_append_multiple_tokens() {
    use proc_macro2::{TokenStream, Ident, Literal};

    let mut tokens = TokenStream::new();
    let tokens_to_append = vec![
        Ident::new("foo").into(),
        Literal::string("bar").into(),
    ];

    for token in tokens_to_append {
        tokens.append(token);
    }

    assert_eq!(tokens.to_string(), "foobar");
}

