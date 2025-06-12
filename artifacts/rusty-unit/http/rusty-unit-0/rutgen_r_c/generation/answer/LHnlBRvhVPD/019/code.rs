// Answer 0

#[test]
fn test_try_append2_with_vacant_space() {
    struct CustomHeaderName(String);
    
    impl Hash for CustomHeaderName {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    impl From<CustomHeaderName> for HeaderName {
        fn from(h: CustomHeaderName) -> Self {
            HeaderName { inner: Repr::Custom(h.0) }
        }
    }

    let mut header_map: HeaderMap<u32> = HeaderMap::with_capacity(1);
    let key = CustomHeaderName("Test-Key".to_string());
    let value = 42;

    let result = header_map.try_append2(key, value);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), false);
    assert_eq!(header_map.len(), 1);
}

#[test]
fn test_try_append2_with_occupied_space() {
    struct CustomHeaderName(String);
    
    impl Hash for CustomHeaderName {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    impl From<CustomHeaderName> for HeaderName {
        fn from(h: CustomHeaderName) -> Self {
            HeaderName { inner: Repr::Custom(h.0) }
        }
    }

    let mut header_map: HeaderMap<u32> = HeaderMap::with_capacity(2);
    let key = CustomHeaderName("Test-Key".to_string());
    let value1 = 42;
    let value2 = 24;

    header_map.try_append2(key.clone(), value1).unwrap();
    let result = header_map.try_append2(key, value2);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true);
    assert_eq!(header_map.len(), 1);
}

#[test]
fn test_try_append2_with_robinhood() {
    struct CustomHeaderName(String);
    
    impl Hash for CustomHeaderName {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    impl From<CustomHeaderName> for HeaderName {
        fn from(h: CustomHeaderName) -> Self {
            HeaderName { inner: Repr::Custom(h.0) }
        }
    }

    let mut header_map: HeaderMap<u32> = HeaderMap::with_capacity(2);
    
    let key1 = CustomHeaderName("Key1".to_string());
    let value1 = 1;

    let key2 = CustomHeaderName("Key2".to_string());
    let value2 = 2;

    // Ensure enough entries so the hash collisions occur
    header_map.try_append2(key1.clone(), value1).unwrap();
    header_map.try_append2(key1.clone(), value2).unwrap();
    
    let result = header_map.try_append2(key2, 3);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), false);
    assert_eq!(header_map.len(), 3);
}

