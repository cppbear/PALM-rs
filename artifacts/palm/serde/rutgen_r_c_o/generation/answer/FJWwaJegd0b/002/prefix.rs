// Answer 0

#[test]
fn test_deserialize_map_valid() {
    let content = Content::Map(vec![
        (Content::String("key1".to_string()), Content::U32(0)),
        (Content::String("key2".to_string()), Content::U64(1)),
    ]);
    let deserializer = ContentDeserializer {
        content,
        err: PhantomData::<value::Error>,
    };
    let visitor = MockVisitor::new();
    let _ = deserializer.deserialize_map(visitor);
}

#[test]
fn test_deserialize_map_empty() {
    let content = Content::Map(vec![]);
    let deserializer = ContentDeserializer {
        content,
        err: PhantomData::<value::Error>,
    };
    let visitor = MockVisitor::new();
    let _ = deserializer.deserialize_map(visitor);
}

#[test]
fn test_deserialize_map_with_none() {
    let content = Content::Map(vec![
        (Content::String("key1".to_string()), Content::None),
    ]);
    let deserializer = ContentDeserializer {
        content,
        err: PhantomData::<value::Error>,
    };
    let visitor = MockVisitor::new();
    let _ = deserializer.deserialize_map(visitor);
}

#[test]
#[should_panic]
fn test_deserialize_map_invalid_type() {
    let content = Content::String("This is not a map".to_string());
    let deserializer = ContentDeserializer {
        content,
        err: PhantomData::<value::Error>,
    };
    let visitor = MockVisitor::new();
    let _ = deserializer.deserialize_map(visitor);
}

