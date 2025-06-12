// Answer 0

#[test]
fn test_insert_entry() {
    use http::header::{HeaderMap, OccupiedEntry, Entry};
    use std::convert::TryInto;

    struct TestValue {
        value: String,
    }

    impl std::str::FromStr for TestValue {
        type Err = std::convert::Infallible;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(TestValue { value: s.to_string() })
        }
    }

    let mut map = HeaderMap::new();

    if let Entry::Vacant(v) = map.try_entry("x-hello").unwrap() {
        let mut e: OccupiedEntry<TestValue> = v.try_insert_entry(TestValue::from_str("world").unwrap()).unwrap();
        e.insert(TestValue::from_str("world2").unwrap());
    }

    assert_eq!(map["x-hello"], "world2");
}

#[test]
fn test_insert_entry_overflow() {
    use http::header::{HeaderMap, Entry};

    struct MockValue;

    let mut map = HeaderMap::new();
    
    // Fill the map to its MAX_SIZE limit before trying to insert a new entry
    for i in 0..map.max_size() {
        map.insert(format!("key-{}", i).as_str(), format!("value-{}", i).as_str().parse().unwrap());
    }

    // Attempting to insert an entry should panic if it overflows
    let result = std::panic::catch_unwind(|| {
        let _ = map.try_entry("overflow-key").unwrap().try_insert_entry(MockValue);
    });

    assert!(result.is_err());
}

