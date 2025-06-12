// Answer 0

#[test]
fn test_try_insert_entry_success() {
    struct TestHeaderMap {
        entries: Vec<Bucket<HeaderValue>>,
    }
    
    impl TestHeaderMap {
        fn new() -> Self {
            TestHeaderMap { entries: Vec::new() }
        }

        fn try_insert_entry(
            &mut self,
            hash: HashValue,
            key: HeaderName,
            value: HeaderValue,
        ) -> Result<(), MaxSizeReached> {
            if self.entries.len() >= MAX_SIZE {
                return Err(MaxSizeReached::new());
            }

            self.entries.push(Bucket {
                hash,
                key,
                value,
                links: None,
            });

            Ok(())
        }
    }

    let mut header_map = TestHeaderMap::new();
    let hash = HashValue(1);
    let key = HeaderName { inner: Repr::Custom }; // Assuming Repr::Custom is a valid state
    let value = HeaderValue::from("TestValue"); // Assuming proper implementation to create HeaderValue

    // Ensure there are no entries initially
    assert_eq!(header_map.entries.len(), 0);
    
    // Insert an entry and check the result
    let result = header_map.try_insert_entry(hash, key.clone(), value);
    assert!(result.is_ok());

    // Check that the entry was added
    assert_eq!(header_map.entries.len(), 1);
} 

#[test]
fn test_try_insert_entry_at_capacity() {
    struct TestHeaderMap {
        entries: Vec<Bucket<HeaderValue>>,
    }
    
    impl TestHeaderMap {
        fn new() -> Self {
            TestHeaderMap { entries: Vec::new() }
        }

        fn try_insert_entry(
            &mut self,
            hash: HashValue,
            key: HeaderName,
            value: HeaderValue,
        ) -> Result<(), MaxSizeReached> {
            if self.entries.len() >= MAX_SIZE {
                return Err(MaxSizeReached::new());
            }

            self.entries.push(Bucket {
                hash,
                key,
                value,
                links: None,
            });

            Ok(())
        }
    }

    // Prepare a header map filled up to MAX_SIZE
    let mut header_map = TestHeaderMap::new();
    let hash = HashValue(1);
    let key = HeaderName { inner: Repr::Custom };
    
    for i in 0..MAX_SIZE {
        let value = HeaderValue::from(format!("Value{}", i));
        header_map.try_insert_entry(HashValue(i as u16), key.clone(), value).unwrap();
    }
    
    // Asserting the entries are full
    assert_eq!(header_map.entries.len(), MAX_SIZE);

    // Attempt to insert another entry should return an error since it's at capacity
    let result = header_map.try_insert_entry(HashValue(MAX_SIZE as u16 + 1), key, HeaderValue::from("OverCapacityValue"));
    assert!(result.is_err());
} 

#[test]
fn test_try_insert_entry_no_panic_on_valid_insertion() {
    struct TestHeaderMap {
        entries: Vec<Bucket<HeaderValue>>,
    }
    
    impl TestHeaderMap {
        fn new() -> Self {
            TestHeaderMap { entries: Vec::new() }
        }

        fn try_insert_entry(
            &mut self,
            hash: HashValue,
            key: HeaderName,
            value: HeaderValue,
        ) -> Result<(), MaxSizeReached> {
            if self.entries.len() >= MAX_SIZE {
                return Err(MaxSizeReached::new());
            }

            self.entries.push(Bucket {
                hash,
                key,
                value,
                links: None,
            });

            Ok(())
        }
    }

    let mut header_map = TestHeaderMap::new();
    let valid_keys: Vec<HeaderName> = (0..10).map(|i| HeaderName { inner: Repr::Custom }).collect();
    
    for (i, key) in valid_keys.iter().enumerate() {
        let value = HeaderValue::from(format!("Value{}", i));
        let hash = HashValue(i as u16);
        let result = header_map.try_insert_entry(hash, key.clone(), value);
        assert!(result.is_ok());
    }

    // Final assertion to confirm no panic and valid entries
    assert_eq!(header_map.entries.len(), valid_keys.len());
}

