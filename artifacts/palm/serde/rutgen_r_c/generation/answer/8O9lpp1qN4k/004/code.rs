// Answer 0

#[test]
fn test_unexpected_content_unit() {
    let content = Content::Unit;
    let unexpected = content.unexpected();
    assert_eq!(unexpected, Unexpected::Unit);
}

#[test]
fn test_unexpected_content_none() {
    let content = Content::None;
    let unexpected = content.unexpected();
    assert_eq!(unexpected, Unexpected::Option);
}

#[test]
fn test_unexpected_content_some() {
    let content = Content::Some(Box::new(Content::Bool(true)));
    let unexpected = content.unexpected();
    assert_eq!(unexpected, Unexpected::Option);
}

