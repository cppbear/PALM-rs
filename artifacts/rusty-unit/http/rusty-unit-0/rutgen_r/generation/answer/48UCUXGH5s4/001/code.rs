// Answer 0

#[test]
fn test_insert_value_into_entry() {
    // Create a new HeaderMap instance
    let mut map = http::header::HeaderMap::new();

    // Insert a key-value pair into a vacant entry
    if let http::header::Entry::Vacant(v) = map.entry("x-hello") {
        let value = "world".parse().unwrap(); // Parsing string to suitable type
        let inserted_value = v.insert(value); // Insert value
        assert_eq!(inserted_value, &mut "world".parse().unwrap()); // Assert mutable reference
    }

    // Verify the value is stored correctly
    assert_eq!(map["x-hello"], "world");
}

#[test]
#[should_panic(expected = "size overflows MAX_SIZE")]
fn test_insert_exceeding_max_size() {
    // Create a new HeaderMap instance
    let mut map = http::header::HeaderMap::new();

    // Fill the HeaderMap to reach max size
    for i in 0..http::header::MAX_SIZE {
        let key = format!("key-{}", i);
        let value = "value".parse().unwrap();
        if let http::header::Entry::Vacant(v) = map.entry(key) {
            v.insert(value);
        }
    }

    // Attempt to insert another value that exceeds max size
    if let http::header::Entry::Vacant(v) = map.entry("overflow") {
        v.insert("should panic".parse().unwrap()); // This should panic
    }
}

