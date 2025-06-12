// Answer 0

#[test]
fn test_unexpected_with_empty_map() {
    let content = Content::Map(Vec::<(Content, Content)>::new());
    content.unexpected();
}

#[test]
fn test_unexpected_with_non_empty_map() {
    let key = Content::String("key".to_string());
    let value = Content::U32(42);
    let content = Content::Map(vec![(key, value)]);
    content.unexpected();
}

#[test]
fn test_unexpected_with_map_of_bytes() {
    let key = Content::Bytes(vec![1, 2, 3]);
    let value = Content::Char('c');
    let content = Content::Map(vec![(key, value)]);
    content.unexpected();
}

#[test]
fn test_unexpected_with_map_of_units() {
    let key = Content::Unit;
    let value = Content::Unit;
    let content = Content::Map(vec![(key, value)]);
    content.unexpected();
}

#[test]
fn test_unexpected_with_map_of_newtype() {
    let key = Content::Newtype(Box::new(Content::String("key".to_string())));
    let value = Content::Newtype(Box::new(Content::I64(100)));
    let content = Content::Map(vec![(key, value)]);
    content.unexpected();
}

