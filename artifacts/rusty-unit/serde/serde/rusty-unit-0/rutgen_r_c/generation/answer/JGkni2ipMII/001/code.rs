// Answer 0

#[test]
fn test_invalid_type_with_bool() {
    struct MockExpected;

    impl Expected for MockExpected {}

    let content = Content::Bool(true);
    let deserializer = ContentDeserializer { content, err: std::marker::PhantomData::<()>::default() };

    let result = deserializer.invalid_type(&MockExpected);

    assert!(result.is_err());
}

#[test]
fn test_invalid_type_with_u8() {
    struct MockExpected;

    impl Expected for MockExpected {}

    let content = Content::U8(255);
    let deserializer = ContentDeserializer { content, err: std::marker::PhantomData::<()>::default() };

    let result = deserializer.invalid_type(&MockExpected);

    assert!(result.is_err());
}

#[test]
fn test_invalid_type_with_string() {
    struct MockExpected;

    impl Expected for MockExpected {}

    let content = Content::String("test".to_string());
    let deserializer = ContentDeserializer { content, err: std::marker::PhantomData::<()>::default() };

    let result = deserializer.invalid_type(&MockExpected);

    assert!(result.is_err());
}

#[test]
fn test_invalid_type_with_seq() {
    struct MockExpected;

    impl Expected for MockExpected {}

    let content = Content::Seq(vec![
        Content::Bool(false),
        Content::U16(300)
    ]);
    let deserializer = ContentDeserializer { content, err: std::marker::PhantomData::<()>::default() };

    let result = deserializer.invalid_type(&MockExpected);

    assert!(result.is_err());
}

#[test]
fn test_invalid_type_with_map() {
    struct MockExpected;

    impl Expected for MockExpected {}

    let content = Content::Map(vec![
        (Content::String("key".to_string()), Content::I32(42)),
        (Content::String("another_key".to_string()), Content::F64(3.14)),
    ]);
    let deserializer = ContentDeserializer { content, err: std::marker::PhantomData::<()>::default() };

    let result = deserializer.invalid_type(&MockExpected);

    assert!(result.is_err());
}

