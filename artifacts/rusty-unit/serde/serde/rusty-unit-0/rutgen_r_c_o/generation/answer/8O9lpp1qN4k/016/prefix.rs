// Answer 0

#[test]
fn test_unexpected_i16_min() {
    let content = Content::I16(-32768);
    content.unexpected();
}

#[test]
fn test_unexpected_i16_lower_half() {
    let content = Content::I16(-100);
    content.unexpected();
}

#[test]
fn test_unexpected_i16_zero() {
    let content = Content::I16(0);
    content.unexpected();
}

#[test]
fn test_unexpected_i16_upper_half() {
    let content = Content::I16(100);
    content.unexpected();
}

#[test]
fn test_unexpected_i16_max() {
    let content = Content::I16(32767);
    content.unexpected();
}

