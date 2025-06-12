// Answer 0

#[test]
fn test_unexpected_for_string_content() {
    let content = Content::String("test string".to_string());
    let result = content.unexpected();
    match result {
        Unexpected::Str(s) => assert_eq!(s, "test string"),
        _ => panic!("Expected Unexpected::Str to be returned"),
    }
}

#[test]
fn test_unexpected_for_string_content_empty() {
    let content = Content::String("".to_string());
    let result = content.unexpected();
    match result {
        Unexpected::Str(s) => assert_eq!(s, ""),
        _ => panic!("Expected Unexpected::Str to be returned"),
    }
}

#[test]
fn test_unexpected_for_string_content_long() {
    let long_string = "a".repeat(1000);
    let content = Content::String(long_string.clone());
    let result = content.unexpected();
    match result {
        Unexpected::Str(s) => assert_eq!(s, long_string),
        _ => panic!("Expected Unexpected::Str to be returned"),
    }
}

