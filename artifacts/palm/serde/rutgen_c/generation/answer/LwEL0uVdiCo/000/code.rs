// Answer 0

#[test]
fn test_end_with_entries() {
    struct TestError;
    impl ser::Error for TestError {}

    let mut serialize_map = SerializeMap {
        entries: vec![
            (Content::String("key1".to_string()), Content::String("value1".to_string())),
            (Content::U32(42), Content::Bool(true)),
        ],
        key: None,
        error: std::marker::PhantomData::<TestError>,
    };

    let result = serialize_map.end().unwrap();
    match result {
        Content::Map(entries) => {
            assert_eq!(entries.len(), 2);
            assert_eq!(entries[0].0, Content::String("key1".to_string()));
            assert_eq!(entries[0].1, Content::String("value1".to_string()));
            assert_eq!(entries[1].0, Content::U32(42));
            assert_eq!(entries[1].1, Content::Bool(true));
        }
        _ => panic!("Expected Content::Map"),
    }
}

#[test]
fn test_end_with_no_entries() {
    struct TestError;
    impl ser::Error for TestError {}

    let serialize_map: SerializeMap<TestError> = SerializeMap {
        entries: vec![],
        key: None,
        error: std::marker::PhantomData,
    };

    let result = serialize_map.end().unwrap();
    match result {
        Content::Map(entries) => {
            assert_eq!(entries.len(), 0);
        }
        _ => panic!("Expected Content::Map"),
    }
}

