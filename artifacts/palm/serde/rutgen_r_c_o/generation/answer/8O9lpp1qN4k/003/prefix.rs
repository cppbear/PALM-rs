// Answer 0

#[test]
fn test_unexpected_newtype_struct_0() {
    let content = Content::Newtype(Box::new(Content::U8(0)));
    content.unexpected();
}

#[test]
fn test_unexpected_newtype_struct_255() {
    let content = Content::Newtype(Box::new(Content::U8(255)));
    content.unexpected();
}

