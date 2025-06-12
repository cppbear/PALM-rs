// Answer 0

#[test]
fn test_unexpected_none() {
    let content = Content::None;
    content.unexpected();
}

#[test]
fn test_unexpected_some_none() {
    let content = Content::Some(Box::new(Content::None));
    content.unexpected();
}

#[test]
fn test_unexpected_some_bool() {
    let content = Content::Some(Box::new(Content::Bool(true)));
    content.unexpected();
}

#[test]
fn test_unexpected_some_u8() {
    let content = Content::Some(Box::new(Content::U8(0)));
    content.unexpected();
}

#[test]
fn test_unexpected_some_string() {
    let content = Content::Some(Box::new(Content::String(String::from("test"))));
    content.unexpected();
}

