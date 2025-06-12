// Answer 0

#[test]
fn test_unexpected_some_bool() {
    let content = Content::Some(Box::new(Content::Bool(true)));
    let _ = content.unexpected();
}

#[test]
fn test_unexpected_some_none() {
    let content = Content::Some(Box::new(Content::None));
    let _ = content.unexpected();
}

#[test]
fn test_unexpected_none() {
    let content = Content::None;
    let _ = content.unexpected();
}

