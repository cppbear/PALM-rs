// Answer 0

#[test]
fn test_unexpected_with_u16() {
    let content_value = Content::U16(42);
    let unexpected_value = content_value.unexpected();
    match unexpected_value {
        Unexpected::Unsigned(n) => assert_eq!(n, 42u64),
        _ => panic!("Expected Unexpected::Unsigned, but got a different variant"),
    }
}

#[test]
fn test_unexpected_with_multiple_u16() {
    let content_value = Content::U16(65535);
    let unexpected_value = content_value.unexpected();
    match unexpected_value {
        Unexpected::Unsigned(n) => assert_eq!(n, 65535u64),
        _ => panic!("Expected Unexpected::Unsigned, but got a different variant"),
    }
}

#[test]
fn test_unexpected_with_min_u16() {
    let content_value = Content::U16(0);
    let unexpected_value = content_value.unexpected();
    match unexpected_value {
        Unexpected::Unsigned(n) => assert_eq!(n, 0u64),
        _ => panic!("Expected Unexpected::Unsigned, but got a different variant"),
    }
}

