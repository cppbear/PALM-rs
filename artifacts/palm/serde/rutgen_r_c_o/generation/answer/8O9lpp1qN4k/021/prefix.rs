// Answer 0

#[test]
fn test_unexpected_u8_0() {
    let content = Content::U8(0);
    content.unexpected();
}

#[test]
fn test_unexpected_u8_255() {
    let content = Content::U8(255);
    content.unexpected();
}

#[test]
fn test_unexpected_u8_128() {
    let content = Content::U8(128);
    content.unexpected();
}

#[test]
fn test_unexpected_u8_1() {
    let content = Content::U8(1);
    content.unexpected();
}

#[test]
fn test_unexpected_u8_100() {
    let content = Content::U8(100);
    content.unexpected();
}

