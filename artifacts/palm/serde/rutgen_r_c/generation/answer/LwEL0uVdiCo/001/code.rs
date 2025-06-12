// Answer 0

#[test]
fn test_end_with_entries() {
    struct DummyError;

    impl serde::Error for DummyError {}

    let mut serialize_map = SerializeMap::<DummyError> {
        entries: vec![
            (Content::String("key1".to_string()), Content::Bool(true)),
            (Content::String("key2".to_string()), Content::U32(42)),
        ],
        key: None,
        error: std::marker::PhantomData,
    };

    let result = serialize_map.end();

    assert!(result.is_ok());
    if let Ok(content) = result {
        match content {
            Content::Map(entries) => {
                assert_eq!(entries.len(), 2);
                assert!(matches!(entries[0], (Content::String(ref k), Content::Bool(_)) if k == "key1"));
                assert!(matches!(entries[1], (Content::String(ref k), Content::U32(_)) if k == "key2"));
            }
            _ => panic!("Expected Content::Map but got a different variant"),
        }
    }
}

#[test]
fn test_end_with_empty_entries() {
    struct DummyError;

    impl serde::Error for DummyError {}

    let serialize_map = SerializeMap::<DummyError> {
        entries: vec![],
        key: None,
        error: std::marker::PhantomData,
    };

    let result = serialize_map.end();

    assert!(result.is_ok());
    if let Ok(content) = result {
        match content {
            Content::Map(entries) => {
                assert_eq!(entries.len(), 0);
            }
            _ => panic!("Expected Content::Map but got a different variant"),
        }
    }
}

