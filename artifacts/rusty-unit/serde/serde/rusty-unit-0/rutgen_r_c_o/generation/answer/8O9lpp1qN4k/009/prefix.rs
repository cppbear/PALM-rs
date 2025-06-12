// Answer 0

#[test]
fn test_unexpected_str_empty() {
    let content = Content::Str("");
    content.unexpected();
}

#[test]
fn test_unexpected_str_short() {
    let content = Content::Str("short");
    content.unexpected();
}

#[test]
fn test_unexpected_str_long() {
    let content = Content::Str("very long string with special characters like !@#$%^&*()_+");
    content.unexpected();
}

#[test]
fn test_unexpected_str_special_chars() {
    let content = Content::Str("!@#$%^&*()_+");
    content.unexpected();
}

#[test]
fn test_unexpected_str_spaces() {
    let content = Content::Str("string with spaces");
    content.unexpected();
}

