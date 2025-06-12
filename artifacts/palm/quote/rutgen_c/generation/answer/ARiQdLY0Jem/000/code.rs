// Answer 0

#[test]
fn test_to_tokens_for_cstring() {
    use proc_macro2::TokenStream;
    use std::ffi::CString;

    let c_string = CString::new("Hello, world!").expect("CString::new failed");
    let mut tokens = TokenStream::new();
    
    c_string.to_tokens(&mut tokens);
    
    let expected = TokenStream::from(Literal::c_string(&c_string));
    assert_eq!(tokens.to_string(), expected.to_string());
}

#[test]
fn test_to_tokens_empty_cstring() {
    use proc_macro2::TokenStream;
    use std::ffi::CString;

    let c_string = CString::new("").expect("CString::new failed");
    let mut tokens = TokenStream::new();
    
    c_string.to_tokens(&mut tokens);
    
    let expected = TokenStream::from(Literal::c_string(&c_string));
    assert_eq!(tokens.to_string(), expected.to_string());
}

#[test]
#[should_panic]
fn test_to_tokens_invalid_cstring() {
    use std::ffi::CString;

    // CString::new will panic if given a null byte
    let _ = CString::new("Hello\0World").expect("CString::new failed");
}

