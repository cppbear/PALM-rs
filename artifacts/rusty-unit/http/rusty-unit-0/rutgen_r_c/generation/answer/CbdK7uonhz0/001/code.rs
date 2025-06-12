// Answer 0

#[test]
fn test_try_entry2_with_max_size_reached() {
    struct CustomHeaderName {
        name: String,
    }

    impl Hash for CustomHeaderName {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.name.hash(state);
        }
    }

    impl Into<HeaderName> for CustomHeaderName {
        fn into(self) -> HeaderName {
            HeaderName { inner: Repr::Custom(self.name) }
        }
    }

    impl PartialEq<CustomHeaderName> for HeaderName {
        fn eq(&self, other: &CustomHeaderName) -> bool {
            self.inner == Repr::Custom(other.name)
        }
    }

    let capacity = 1; // Start with a low capacity
    let mut header_map: HeaderMap<String> = HeaderMap::with_capacity(capacity);
    
    // Fill the header map to its capacity
    header_map.insert(CustomHeaderName { name: "Test".to_string() }, "Value1".to_string());

    // Now try to reserve one more which should lead to MaxSizeReached
    let result = header_map.try_entry2(CustomHeaderName { name: "Test2".to_string() });
    
    match result {
        Err(_) => {}, // Expected to reach here, indicating max size reached
        Ok(_) => panic!("Expected MaxSizeReached, but got an Entry."),
    }
}  

#[test]
fn test_try_entry2_with_space_available() {
    struct CustomHeaderName {
        name: String,
    }

    impl Hash for CustomHeaderName {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.name.hash(state);
        }
    }

    impl Into<HeaderName> for CustomHeaderName {
        fn into(self) -> HeaderName {
            HeaderName { inner: Repr::Custom(self.name) }
        }
    }

    impl PartialEq<CustomHeaderName> for HeaderName {
        fn eq(&self, other: &CustomHeaderName) -> bool {
            self.inner == Repr::Custom(other.name)
        }
    }

    let capacity = 2; // Start with a capacity that allows more entries
    let mut header_map: HeaderMap<String> = HeaderMap::with_capacity(capacity);
    
    // Fill part of the header map
    header_map.insert(CustomHeaderName { name: "Test".to_string() }, "Value1".to_string());

    // Now try to add another entry which should be successful
    let result = header_map.try_entry2(CustomHeaderName { name: "Test2".to_string() });

    match result {
        Ok(entry) => match entry {
            Entry::Vacant(_) => {}, // Expected to get a Vacant entry
            _ => panic!("Expected a Vacant entry but got Occupied."),
        },
        Err(_) => panic!("Expected successfully entry but got MaxSizeReached."),
    }
}

