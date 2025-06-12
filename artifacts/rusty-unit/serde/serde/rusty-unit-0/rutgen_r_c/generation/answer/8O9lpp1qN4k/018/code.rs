// Answer 0

#[test]
fn test_unexpected_content_u64() {
    let content_u64 = Content::U64(42);
    let unexpected_result = content_u64.unexpected();
    assert_eq!(unexpected_result, Unexpected::Unsigned(42));
}

#[test]
fn test_unexpected_content_none() {
    let content_none = Content::None;
    let unexpected_result = content_none.unexpected();
    assert_eq!(unexpected_result, Unexpected::Option);
}

#[test]
fn test_unexpected_content_some() {
    let content_some = Content::Some(Box::new(Content::U32(10)));
    let unexpected_result = content_some.unexpected();
    assert_eq!(unexpected_result, Unexpected::Option);
}

#[test]
fn test_unexpected_content_bool() {
    let content_bool = Content::Bool(true);
    let unexpected_result = content_bool.unexpected();
    assert_eq!(unexpected_result, Unexpected::Bool(true));
}

