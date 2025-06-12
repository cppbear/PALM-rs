// Answer 0

#[test]
fn test_to_tokens_with_cstr() {
    use std::ffi::CStr;
    use proc_macro2::{TokenStream, Literal};

    let cstring = CString::new("Hello, world!").unwrap();
    let cstr: &CStr = cstring.as_c_str();

    let mut tokens = TokenStream::new();
    cstr.to_tokens(&mut tokens);

    let expected_tokens = TokenStream::from(Literal::c_string(cstr));
    assert_eq!(tokens.to_string(), expected_tokens.to_string());
}

#[test]
fn test_to_tokens_with_empty_cstr() {
    use std::ffi::CStr;
    use proc_macro2::{TokenStream, Literal};

    let empty_cstring = CString::new("").unwrap();
    let empty_cstr: &CStr = empty_cstring.as_c_str();

    let mut tokens = TokenStream::new();
    empty_cstr.to_tokens(&mut tokens);

    let expected_tokens = TokenStream::from(Literal::c_string(empty_cstr));
    assert_eq!(tokens.to_string(), expected_tokens.to_string());
}

