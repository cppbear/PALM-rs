// Answer 0

#[test]
fn test_remove_found_happy_path() {
    struct TestValue;
    struct TestHeaderMap {
        map: HeaderMap<TestValue>,
    }

    impl TestHeaderMap {
        fn new() -> Self {
            let mut map = HeaderMap::with_capacity(4);
            // Add a dummy entry for removal
            map.insert("Key1", TestValue);
            map.insert("Key2", TestValue);
            map.insert("Key3", TestValue);
            Self { map }
        }

        fn setup_for_removal(&mut self) -> (usize, usize) {
            let found = 1; // Index for "Key2"
            let probe = found; // Probe index corresponding to found
            (probe, found)
        }
    }

    let mut header_map = TestHeaderMap::new();
    let (probe, found) = header_map.setup_for_removal();
    let bucket = header_map.map.remove_found(probe, found);
    assert!(bucket.key == HeaderName::try_from("Key2").unwrap());
}

#[test]
#[should_panic] // Expect to panic because found index is out of bounds
fn test_remove_found_out_of_bounds() {
    struct TestValue;
    struct TestHeaderMap {
        map: HeaderMap<TestValue>,
    }

    impl TestHeaderMap {
        fn new() -> Self {
            let mut map = HeaderMap::with_capacity(2);
            // Add two dummy entries
            map.insert("Key1", TestValue);
            map.insert("Key2", TestValue);
            Self { map }
        }
    }

    let mut header_map = TestHeaderMap::new();
    let probe = 0; // Valid probe index
    let found = 2; // Out of bounds found index
    header_map.map.remove_found(probe, found);
}

#[test]
fn test_remove_found_empty_entries() {
    struct TestValue;
    struct TestHeaderMap {
        map: HeaderMap<TestValue>,
    }

    impl TestHeaderMap {
        fn new() -> Self {
            let map = HeaderMap::with_capacity(0);
            Self { map }
        }
    }

    let mut header_map = TestHeaderMap::new();
    let probe = 0; // Valid probe index (though map is empty)
    let found = 0; // Found index (also empty)
    // calling remove_found on an empty map should not panic, 
    // will just return a default Bucket.
    let bucket = header_map.map.remove_found(probe, found);
    assert_eq!(bucket.hash, HashValue(0)); // Bucket should be default
}

