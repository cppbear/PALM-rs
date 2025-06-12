// Answer 0

#[test]
fn test_try_insert2_vacant() {
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

    let mut header_map: HeaderMap<String> = HeaderMap::with_capacity(2);
    let key = CustomHeaderName { name: "Test-Header".to_string() };
    let value = "TestValue".to_string();

    header_map.try_insert_entry(HashValue(1), key.into(), value.clone()).unwrap();
    let result = header_map.try_insert2(key, value);
    
    assert_eq!(result, Ok(None));
    assert_eq!(header_map.len(), 1);
}

#[test]
fn test_try_insert2_occupied() {
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

    let mut header_map: HeaderMap<String> = HeaderMap::with_capacity(2);
    let key1 = CustomHeaderName { name: "Test-Header".to_string() };
    let value1 = "TestValue1".to_string();
    let key2 = CustomHeaderName { name: "Test-Header".to_string() };
    let value2 = "TestValue2".to_string();

    header_map.try_insert_entry(HashValue(1), key1.into(), value1.clone()).unwrap();
    let result = header_map.try_insert2(key2, value2.clone());
    
    assert_eq!(result, Ok(Some(value1)));
    assert_eq!(header_map.len(), 1);
}

#[test]
fn test_try_insert2_robinhood() {
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

    let mut header_map: HeaderMap<String> = HeaderMap::with_capacity(2);
    let key1 = CustomHeaderName { name: "Test-Header1".to_string() };
    let value1 = "Value1".to_string();
    let key2 = CustomHeaderName { name: "Test-Header2".to_string() };
    let value2 = "Value2".to_string();

    header_map.try_insert_entry(HashValue(1), key1.clone().into(), value1.clone()).unwrap();
    header_map.try_insert_entry(HashValue(2), key2.clone().into(), value2.clone()).unwrap();
    let result = header_map.try_insert2(key1, value1.clone());
    
    assert_eq!(result, Ok(Some(value2)));
    assert_eq!(header_map.len(), 2);
}

