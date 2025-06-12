// Answer 0

#[test]
fn test_serialize_map_with_none_length() {
    let serializer = Serializer;
    let result = serializer.serialize_map(None).unwrap();
    match result {
        SerializeMap::Map { map, next_key } => {
            assert!(map.is_empty());
            assert!(next_key.is_none());
        }
        _ => panic!("Expected SerializeMap::Map"),
    }
}

#[test]
fn test_serialize_map_with_zero_length() {
    let serializer = Serializer;
    let result = serializer.serialize_map(Some(0)).unwrap();
    match result {
        SerializeMap::Map { map, next_key } => {
            assert!(map.is_empty());
            assert!(next_key.is_none());
        }
        _ => panic!("Expected SerializeMap::Map"),
    }
}

#[test]
fn test_serialize_map_with_positive_length() {
    let serializer = Serializer;
    let result = serializer.serialize_map(Some(10)).unwrap();
    match result {
        SerializeMap::Map { map, next_key } => {
            assert!(map.is_empty());
            assert!(next_key.is_none());
        }
        _ => panic!("Expected SerializeMap::Map"),
    }
}

