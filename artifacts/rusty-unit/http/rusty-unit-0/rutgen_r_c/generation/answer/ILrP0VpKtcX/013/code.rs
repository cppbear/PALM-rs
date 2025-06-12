// Answer 0

#[test]
fn test_remove_found_with_valid_entry() {
    struct TestHeaderValue;
    impl fmt::Debug for TestHeaderValue {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "TestHeaderValue")
        }
    }

    let mut header_map: HeaderMap<TestHeaderValue> = HeaderMap::with_capacity(2);
    let key = HeaderName::from_static("Test-Header");
    let value = TestHeaderValue;

    // Insert a test entry directly to the HeaderMap
    header_map.entries.push(Bucket {
        hash: HashValue(1),
        key,
        value: value,
        links: Some(Links { next: 0, tail: 0 }),
    });
    header_map.indices = Box::new([Pos::new(0, HashValue(1))]);
    let probe = 0;
    let found = 0;

    // Call remove_found with a known good probe and found index
    let entry = header_map.remove_found(probe, found);

    // Assert that the removed entry matches the initial entry
    assert_eq!(entry.key, Key::from_static("Test-Header"));
    assert_eq!(entry.value.debug(), "TestHeaderValue");
}

#[test]
#[should_panic]
fn test_remove_found_with_empty_entries() {
    struct TestHeaderValue;
    let mut header_map: HeaderMap<TestHeaderValue> = HeaderMap::with_capacity(1);
    let probe = 0;
    let found = 0;

    // This should panic since entries is empty
    header_map.remove_found(probe, found);
}

#[test]
fn test_remove_found_with_non_last_element() {
    struct TestHeaderValue;
    impl fmt::Debug for TestHeaderValue {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "TestHeaderValue")
        }
    }

    // Setup HeaderMap with multiple entries
    let mut header_map: HeaderMap<TestHeaderValue> = HeaderMap::with_capacity(2);
    
    // Add two test entries to enforce conditions in remove_found
    header_map.entries.push(Bucket {
        hash: HashValue(1),
        key: HeaderName::from_static("Header-One"),
        value: TestHeaderValue,
        links: Some(Links { next: 1, tail: 1 }),
    });
    
    header_map.entries.push(Bucket {
        hash: HashValue(2),
        key: HeaderName::from_static("Header-Two"),
        value: TestHeaderValue,
        links: Some(Links { next: 0, tail: 0 }),
    });

    header_map.indices = Box::new([Pos::new(0, HashValue(1)), Pos::new(1, HashValue(2))]);
    let probe = 0; // Test removing the first entry
    let found = 0; // Should return the first entry

    // Call remove_found with a valid probe and found index
    let entry = header_map.remove_found(probe, found);

    // Assert removed entry is as expected
    assert_eq!(entry.key, HeaderName::from_static("Header-One"));
}

#[test]
#[should_panic]
fn test_remove_found_with_invalid_probe() {
    struct TestHeaderValue;
    let mut header_map: HeaderMap<TestHeaderValue> = HeaderMap::with_capacity(2);
    
    // Prepare valid entries for testing
    let key = HeaderName::from_static("Header-At-Probe");
    let value = TestHeaderValue;

    header_map.entries.push(Bucket {
        hash: HashValue(1),
        key,
        value: value,
        links: None,
    });
    header_map.indices = Box::new([Pos::new(0, HashValue(1))]);

    // Set an invalid probe for removal
    let probe = 1; // Out of bounds
    let found = 0;

    // This should panic due to invalid probe handling
    header_map.remove_found(probe, found);
}

