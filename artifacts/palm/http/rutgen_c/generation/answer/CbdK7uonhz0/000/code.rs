// Answer 0

#[test]
fn test_try_entry2_vacant_entry() {
    struct TestHeaderName {
        name: String,
    }

    impl Hash for TestHeaderName {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.name.hash(state);
        }
    }

    impl PartialEq for TestHeaderName {
        fn eq(&self, other: &Self) -> bool {
            self.name == other.name
        }
    }

    impl Into<HeaderName> for TestHeaderName {
        fn into(self) -> HeaderName {
            HeaderName { inner: Repr::Custom(self.name.into_bytes()) }
        }
    }

    let mut header_map: HeaderMap<HeaderValue> = HeaderMap::with_capacity(10);
    let key = TestHeaderName { name: "Test-Key".to_string() };

    let entry = header_map.try_entry2(key).unwrap();
    match entry {
        Entry::Vacant(_) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_try_entry2_edge_case() {
    struct TestHeaderName {
        name: String,
    }

    impl Hash for TestHeaderName {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.name.hash(state);
        }
    }

    impl PartialEq for TestHeaderName {
        fn eq(&self, other: &Self) -> bool {
            self.name == other.name
        }
    }

    impl Into<HeaderName> for TestHeaderName {
        fn into(self) -> HeaderName {
            HeaderName { inner: Repr::Custom(self.name.into_bytes()) }
        }
    }

    let mut header_map: HeaderMap<HeaderValue> = HeaderMap::with_capacity(0); // Initialize with 0 capacity
    let key = TestHeaderName { name: "Edge-Key".to_string() };

    let result = header_map.try_entry2(key);
    assert!(result.is_err());
}

#[test]
fn test_try_entry2_full_capacity() {
    struct TestHeaderName {
        name: String,
    }

    impl Hash for TestHeaderName {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.name.hash(state);
        }
    }

    impl PartialEq for TestHeaderName {
        fn eq(&self, other: &Self) -> bool {
            self.name == other.name
        }
    }

    impl Into<HeaderName> for TestHeaderName {
        fn into(self) -> HeaderName {
            HeaderName { inner: Repr::Custom(self.name.into_bytes()) }
        }
    }

    let mut header_map: HeaderMap<HeaderValue> = HeaderMap::with_capacity(1);
    let key1 = TestHeaderName { name: "Key1".to_string() };
    let key2 = TestHeaderName { name: "Key2".to_string() };

    header_map.try_entry2(key1).unwrap(); // Fill the map

    let result = header_map.try_entry2(key2); // This should cause a MaxSizeReached
    assert!(result.is_err());
}

