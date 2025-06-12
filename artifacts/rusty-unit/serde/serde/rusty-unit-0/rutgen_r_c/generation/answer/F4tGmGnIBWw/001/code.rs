// Answer 0

#[test]
fn test_map_access_deserializer_new() {
    struct DummyMap;
    
    let map_instance = DummyMap;
    let map_access_deserializer = MapAccessDeserializer::new(map_instance);
    
    assert_eq!(std::mem::size_of_val(&map_access_deserializer), std::mem::size_of::<MapAccessDeserializer<DummyMap>>());
}

#[test]
fn test_map_access_deserializer_empty() {
    struct EmptyMap;

    let empty_map_instance = EmptyMap;
    let map_access_deserializer = MapAccessDeserializer::new(empty_map_instance);

    assert_eq!(std::mem::size_of_val(&map_access_deserializer), std::mem::size_of::<MapAccessDeserializer<EmptyMap>>());
}

#[test]
fn test_map_access_deserializer_large_map() {
    struct LargeMap {
        data: [u8; 1024],
    }

    let large_map_instance = LargeMap { data: [0; 1024] };
    let map_access_deserializer = MapAccessDeserializer::new(large_map_instance);

    assert_eq!(std::mem::size_of_val(&map_access_deserializer), std::mem::size_of::<MapAccessDeserializer<LargeMap>>());
}

