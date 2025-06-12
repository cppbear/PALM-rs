// Answer 0

#[test]
fn test_map_deserializer_new() {
    struct DummyPair;
    
    impl private::Pair for DummyPair {}

    let iter = vec![DummyPair, DummyPair].into_iter();
    let deserializer: MapDeserializer<_, ()> = MapDeserializer::new(iter);
    
    assert!(deserializer.count == 0);
    assert!(deserializer.value.is_none());
}

#[test]
fn test_map_deserializer_new_with_empty_iter() {
    struct DummyPair;

    impl private::Pair for DummyPair {}

    let iter: Vec<DummyPair> = vec![];
    let deserializer: MapDeserializer<_, ()> = MapDeserializer::new(iter.into_iter());

    assert!(deserializer.count == 0);
    assert!(deserializer.value.is_none());
}

