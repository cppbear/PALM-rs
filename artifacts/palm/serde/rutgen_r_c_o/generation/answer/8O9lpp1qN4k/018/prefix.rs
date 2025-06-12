// Answer 0

#[test]
fn test_unexpected_with_u64_min_value() {
    let content = Content::U64(0);
    content.unexpected();
}

#[test]
fn test_unexpected_with_u64_mid_value() {
    let content = Content::U64(9223372036854775807);
    content.unexpected();
}

#[test]
fn test_unexpected_with_u64_max_value() {
    let content = Content::U64(18446744073709551615);
    content.unexpected();
}

