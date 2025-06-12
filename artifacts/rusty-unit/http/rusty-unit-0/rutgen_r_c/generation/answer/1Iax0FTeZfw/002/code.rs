// Answer 0

#[test]
fn test_try_from_generic_invalid_character() {
    struct TestInput<'a>(&'a [u8]);

    let input = TestInput(b"Invalid\x00Character");
    let result: Result<HeaderValue, InvalidHeaderValue> = HeaderValue::try_from_generic(input, |src| Bytes::from(src.0));

    assert!(result.is_err());
}

#[test]
fn test_try_from_generic_invalid_ascii_character() {
    struct TestInput<'a>(&'a [u8]);

    let input = TestInput(b"InvalidCharacter\x7F");
    let result: Result<HeaderValue, InvalidHeaderValue> = HeaderValue::try_from_generic(input, |src| Bytes::from(src.0));

    assert!(result.is_err());
}

#[test]
fn test_try_from_generic_only_valid_characters() {
    struct TestInput<'a>(&'a [u8]);

    let input = TestInput(b"Valid\tCharacters");
    let result: Result<HeaderValue, InvalidHeaderValue> = HeaderValue::try_from_generic(input, |src| Bytes::from(src.0));

    assert!(result.is_ok());
}

