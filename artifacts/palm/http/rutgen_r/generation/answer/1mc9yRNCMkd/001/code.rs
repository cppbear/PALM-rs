// Answer 0

#[test]
fn test_try_insert_vacant_entry() {
    use http::header::{HeaderMap, Entry};

    let mut map = HeaderMap::new();

    if let Entry::Vacant(v) = map.entry("x-hello") {
        let result = v.try_insert("world".parse::<http::header::HeaderValue>().unwrap());
        assert_eq!(result.is_ok(), true);

        let value = result.unwrap();
        assert_eq!(*value, "world".parse::<http::header::HeaderValue>().unwrap());
    }
}

#[test]
fn test_try_insert_exceeding_max_size() {
    use http::header::{HeaderMap, Entry};
    use http::header::MaxSizeReached;

    let mut map = HeaderMap::new();

    // Fill the map to the max size (assuming max size is predefined or known)
    for i in 0..1000 { // Example size limit
        map.insert(format!("key-{}", i), "value".parse().unwrap());
    }

    if let Entry::Vacant(v) = map.entry("x-full") {
        let result = v.try_insert("new_value".parse::<http::header::HeaderValue>().unwrap());
        assert!(result.is_err());

        if let Err(MaxSizeReached) = result {
            // Test confirms that MaxSizeReached was returned
            assert!(true);
        } else {
            assert!(false, "Expected MaxSizeReached error");
        }
    }
}

