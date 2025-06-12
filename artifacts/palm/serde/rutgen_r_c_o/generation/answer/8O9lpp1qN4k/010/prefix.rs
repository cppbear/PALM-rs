// Answer 0

#[test]
fn test_unexpected_string_empty() {
    let content = Content::String(String::new());
    let _ = content.unexpected();
}

#[test]
fn test_unexpected_string_normal() {
    let content = Content::String("Hello, World!".to_string());
    let _ = content.unexpected();
}

#[test]
fn test_unexpected_string_special_characters() {
    let content = Content::String("!@#$%^&*()".to_string());
    let _ = content.unexpected();
}

#[test]
fn test_unexpected_string_whitespace() {
    let content = Content::String("   ".to_string());
    let _ = content.unexpected();
}

#[test]
fn test_unexpected_string_unicode() {
    let content = Content::String("Привет, мир".to_string());
    let _ = content.unexpected();
}

#[test]
fn test_unexpected_string_max_length() {
    let content = Content::String("a".repeat(256));
    let _ = content.unexpected();
}

