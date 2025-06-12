// Answer 0


use quote::ToTokens;
use proc_macro2::TokenStream;

struct TestStruct;

impl ToTokens for TestStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(quote::quote! { TestStruct });
    }
}

#[test]
fn test_to_tokens_valid() {
    let instance = TestStruct;
    let mut tokens = TokenStream::new();
    
    instance.to_tokens(&mut tokens);
    
    let expected_tokens = quote::quote! { TestStruct };
    assert_eq!(tokens.to_string(), expected_tokens.to_string());
}

#[should_panic]
#[test]
fn test_to_tokens_panic() {
    struct PanickingStruct;

    impl ToTokens for PanickingStruct {
        fn to_tokens(&self, _: &mut TokenStream) {
            panic!("This should panic");
        }
    }

    let instance = PanickingStruct;
    let mut tokens = TokenStream::new();
    
    instance.to_tokens(&mut tokens);
}


