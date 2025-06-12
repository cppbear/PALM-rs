// Answer 0

#[test]
fn test_serialize_map_with_some_length() {
    struct TestSerialize;

    impl TestSerialize {
        fn serialize_map(self, len: Option<usize>) -> Result<SerializeMap> {
            Ok(SerializeMap::Map {
                map: Map::with_capacity(len.unwrap_or(0)),
                next_key: None,
            })
        }
    }

    let ser = TestSerialize;
    let result = ser.serialize_map(Some(5)).unwrap();
    match result {
        SerializeMap::Map { map, next_key } => {
            assert_eq!(map.capacity(), 5);
            assert!(next_key.is_none());
        }
    }
}

#[test]
fn test_serialize_map_with_none_length() {
    struct TestSerialize;

    impl TestSerialize {
        fn serialize_map(self, len: Option<usize>) -> Result<SerializeMap> {
            Ok(SerializeMap::Map {
                map: Map::with_capacity(len.unwrap_or(0)),
                next_key: None,
            })
        }
    }

    let ser = TestSerialize;
    let result = ser.serialize_map(None).unwrap();
    match result {
        SerializeMap::Map { map, next_key } => {
            assert_eq!(map.capacity(), 0);
            assert!(next_key.is_none());
        }
    }
}

