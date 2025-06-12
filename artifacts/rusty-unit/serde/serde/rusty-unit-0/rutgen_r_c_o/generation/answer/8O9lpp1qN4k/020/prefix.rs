// Answer 0

#[test]
fn test_unexpected_content_u16_min() {
    let content = Content::U16(0);
    content.unexpected();
}

#[test]
fn test_unexpected_content_u16_mid() {
    let content = Content::U16(32768);
    content.unexpected();
}

#[test]
fn test_unexpected_content_u16_max() {
    let content = Content::U16(65535);
    content.unexpected();
}

