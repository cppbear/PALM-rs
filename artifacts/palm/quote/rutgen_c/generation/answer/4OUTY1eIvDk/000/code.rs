// Answer 0

#[test]
fn test_to_tokens_rc() {
    struct TestStruct;

    impl ToTokens for TestStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.extend(quote::quote! { TestStruct });
        }
    }

    let test_instance = Rc::new(TestStruct);
    let mut token_stream = TokenStream::new();
    
    test_instance.to_tokens(&mut token_stream);
    
    let expected = quote::quote! { TestStruct };
    assert_eq!(token_stream.to_string(), expected.to_string());
}

#[test]
fn test_to_token_stream_rc() {
    struct TestStruct;

    impl ToTokens for TestStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.extend(quote::quote! { TestStruct });
        }
    }

    let test_instance = Rc::new(TestStruct);
    
    let token_stream = test_instance.to_token_stream();
    
    let expected = quote::quote! { TestStruct };
    assert_eq!(token_stream.to_string(), expected.to_string());
}

#[test]
fn test_into_token_stream_rc() {
    struct TestStruct;

    impl ToTokens for TestStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.extend(quote::quote! { TestStruct });
        }
    }

    let test_instance = Rc::new(TestStruct);
    
    let token_stream = test_instance.clone().into_token_stream();
    
    let expected = quote::quote! { TestStruct };
    assert_eq!(token_stream.to_string(), expected.to_string());
}

