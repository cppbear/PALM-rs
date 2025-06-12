// Answer 0

#[test]
fn test_to_tokens_with_valid_cstring() {
    use proc_macro2::TokenStream;
    use std::ffi::CString;

    let valid_cstring = CString::new("Hello, world!").expect("Failed to create CString");
    let mut tokens = TokenStream::new();

    valid_cstring.to_tokens(&mut tokens);

    let expected_output = "Hello, world!";
    assert!(tokens.to_string().contains(expected_output), "Expected output not found in tokens");
}

#[test]
#[should_panic(expected = "NUL byte found in CString")]
fn test_to_tokens_with_invalid_cstring() {
    use std::ffi::CString;

    // CString creation with a NUL byte which should panic
    let _invalid_cstring = CString::new("Hello\0world").unwrap();
}

