// Answer 0

#[test]
fn test_serialize_map_with_some_length() {
    struct SerializeMap;
    struct Map {
        capacity: usize,
    }

    impl Map {
        fn with_capacity(capacity: usize) -> Self {
            Map { capacity }
        }
    }

    impl SerializeMap {
        fn serialize_map(len: Option<usize>) -> Result<Self, ()> {
            Ok(SerializeMap::Map {
                map: Map::with_capacity(len.unwrap_or(0)),
                next_key: None,
            })
        }
    }

    let result = SerializeMap::serialize_map(Some(5)).unwrap();
    if let SerializeMap::Map { map, .. } = result {
        assert_eq!(map.capacity, 5);
    } else {
        panic!("Expected SerializeMap::Map variant");
    }
}

#[test]
fn test_serialize_map_with_none_length() {
    struct SerializeMap;
    struct Map {
        capacity: usize,
    }

    impl Map {
        fn with_capacity(capacity: usize) -> Self {
            Map { capacity }
        }
    }

    impl SerializeMap {
        fn serialize_map(len: Option<usize>) -> Result<Self, ()> {
            Ok(SerializeMap::Map {
                map: Map::with_capacity(len.unwrap_or(0)),
                next_key: None,
            })
        }
    }

    let result = SerializeMap::serialize_map(None).unwrap();
    if let SerializeMap::Map { map, .. } = result {
        assert_eq!(map.capacity, 0);
    } else {
        panic!("Expected SerializeMap::Map variant");
    }
}

