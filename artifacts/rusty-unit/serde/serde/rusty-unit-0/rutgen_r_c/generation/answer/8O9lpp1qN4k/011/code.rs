// Answer 0

#[test]
fn test_unexpected_with_char() {
    let content = Content::Char('a');
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Char('a'));
}

#[test]
fn test_unexpected_with_char_boundary() {
    let content = Content::Char('z');
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Char('z'));
}

