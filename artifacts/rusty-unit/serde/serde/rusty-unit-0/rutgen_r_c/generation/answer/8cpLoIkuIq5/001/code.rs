// Answer 0

#[test]
fn test_into_deserializer_bool() {
    let content = Content::Bool(true);
    let deserializer = content.into_deserializer();
    assert_eq!(deserializer.content, Content::Bool(true));
}

#[test]
fn test_into_deserializer_u8() {
    let content = Content::U8(255);
    let deserializer = content.into_deserializer();
    assert_eq!(deserializer.content, Content::U8(255));
}

#[test]
fn test_into_deserializer_string() {
    let content = Content::String(String::from("test"));
    let deserializer = content.into_deserializer();
    assert_eq!(deserializer.content, Content::String(String::from("test")));
}

#[test]
fn test_into_deserializer_seq() {
    let content = Content::Seq(vec![Content::U32(1), Content::U32(2)]);
    let deserializer = content.into_deserializer();
    assert_eq!(deserializer.content, Content::Seq(vec![Content::U32(1), Content::U32(2)]));
}

#[test]
fn test_into_deserializer_map() {
    let content = Content::Map(vec![
        (Content::String(String::from("key")), Content::U32(10)),
        (Content::String(String::from("key2")), Content::U32(20)),
    ]);
    let deserializer = content.into_deserializer();
    assert_eq!(deserializer.content, Content::Map(vec![
        (Content::String(String::from("key")), Content::U32(10)),
        (Content::String(String::from("key2")), Content::U32(20)),
    ]));
}

#[test]
fn test_into_deserializer_none() {
    let content = Content::None;
    let deserializer = content.into_deserializer();
    assert_eq!(deserializer.content, Content::None);
}

#[test]
fn test_into_deserializer_unit() {
    let content = Content::Unit;
    let deserializer = content.into_deserializer();
    assert_eq!(deserializer.content, Content::Unit);
}

