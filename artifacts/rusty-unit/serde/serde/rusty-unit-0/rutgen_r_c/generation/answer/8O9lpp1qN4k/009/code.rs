// Answer 0

#[test]
fn test_unexpected_str() {
    let content = Content::Str("test string");
    let unexpected_result = content.unexpected();
    assert_eq!(unexpected_result, Unexpected::Str("test string"));
}

#[test]
fn test_unexpected_string() {
    let content = Content::String(String::from("test string"));
    let unexpected_result = content.unexpected();
    assert_eq!(unexpected_result, Unexpected::Str("test string"));
}

#[test]
fn test_unexpected_str_empty() {
    let content = Content::Str("");
    let unexpected_result = content.unexpected();
    assert_eq!(unexpected_result, Unexpected::Str(""));
}

#[test]
fn test_unexpected_string_empty() {
    let content = Content::String(String::from(""));
    let unexpected_result = content.unexpected();
    assert_eq!(unexpected_result, Unexpected::Str(""));
}

