// Answer 0

#[test]
fn test_serialize_value_with_some_content() {
    struct TestError;
    impl ser::Error for TestError {}

    let mut map = SerializeMap {
        entries: Vec::new(),
        key: Some(Content::String("key".into())),
        error: PhantomData::<TestError>,
    };

    let result = map.serialize_value(&Content::U32(42));
    assert!(result.is_ok());
    assert_eq!(map.entries.len(), 1);
    assert_eq!(map.entries[0].0, Content::String("key".into()));
    assert_eq!(map.entries[0].1, Content::U32(42));
}

#[test]
fn test_serialize_value_with_none_key() {
    struct TestError;
    impl ser::Error for TestError {}

    let mut map = SerializeMap {
        entries: Vec::new(),
        key: None,
        error: PhantomData::<TestError>,
    };

    let result = map.serialize_value(&Content::U32(42));
    assert!(result.is_err());
}

#[test]
fn test_serialize_value_sequential_calls() {
    struct TestError;
    impl ser::Error for TestError {}

    let mut map = SerializeMap {
        entries: Vec::new(),
        key: Some(Content::String("first_key".into())),
        error: PhantomData::<TestError>,
    };

    let result1 = map.serialize_value(&Content::U32(42));
    assert!(result1.is_ok());
    assert_eq!(map.entries.len(), 1);

    map.key = Some(Content::String("second_key".into()));
    let result2 = map.serialize_value(&Content::Bool(true));
    assert!(result2.is_ok());
    assert_eq!(map.entries.len(), 2);

    assert_eq!(map.entries[1].0, Content::String("second_key".into()));
    assert_eq!(map.entries[1].1, Content::Bool(true));
}

