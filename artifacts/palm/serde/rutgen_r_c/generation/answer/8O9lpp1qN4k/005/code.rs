// Answer 0

#[test]
fn test_unexpected_some() {
    let content = Content::Some(Box::new(Content::Bool(true)));
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Option);
}

#[test]
fn test_unexpected_none() {
    let content = Content::None;
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Option);
}

