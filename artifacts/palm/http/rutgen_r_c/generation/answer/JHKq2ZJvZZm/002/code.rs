// Answer 0

#[test]
fn test_try_from_invalid_scheme() {
    let input: &[u8] = b"invalid_scheme";
    let result: Result<Scheme, InvalidUri> = TryFrom::try_from(input);
    assert!(result.is_err());
}

#[test]
fn test_try_from_valid_scheme() {
    let input: &[u8] = b"custom_scheme";
    let result: Result<Scheme, InvalidUri> = TryFrom::try_from(input);
    
    match result {
        Ok(scheme) => {
            match scheme.inner {
                Scheme2::Other(ref byte_str) => {
                    let expected_bytes = Bytes::copy_from_slice(b"custom_scheme");
                    let expected_string = unsafe { ByteStr::from_utf8_unchecked(expected_bytes) };
                    assert_eq!(*byte_str, expected_string);
                },
                _ => panic!("Expected Scheme2::Other variant"),
            }
        },
        Err(_) => panic!("Expected Ok but got Err"),
    }
}

#[test]
fn test_try_from_scheme_too_long() {
    let input = b"this_scheme_is_way_too_long_to_be_valid_and_should_trigger_an_error";
    let result: Result<Scheme, InvalidUri> = TryFrom::try_from(input);
    assert!(result.is_err());
}

