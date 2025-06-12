// Answer 0

#[test]
fn test_no_expansion_with_no_dollar_sign() {
    let input: &[u8] = b"hello world";
    let result = input.no_expansion();
    assert_eq!(result, Some(Cow::Borrowed(b"hello world")));
}

#[test]
fn test_no_expansion_with_dollar_sign() {
    let input: &[u8] = b"hello $world";
    let result = input.no_expansion();
    assert_eq!(result, None);
}

#[test]
fn test_no_expansion_empty_string() {
    let input: &[u8] = b"";
    let result = input.no_expansion();
    assert_eq!(result, Some(Cow::Borrowed(b"")));
}

