// Answer 0

#[test]
fn test_try_append_success_new_key() {
    struct DummyHeaderName(String);
    
    impl IntoHeaderName for DummyHeaderName {
        fn try_append(self, map: &mut HeaderMap<String>, value: String) -> Result<bool, MaxSizeReached> {
            map.insert(self.0.clone(), value);
            Ok(false)
        }
    }
    
    let mut map = HeaderMap::with_capacity(10);
    let result = map.try_append(DummyHeaderName("Host".to_string()), "world".to_string()).unwrap();
    assert!(!result);
    assert!(!map.is_empty());
}

#[test]
fn test_try_append_success_existing_key() {
    struct DummyHeaderName(String);
    
    impl IntoHeaderName for DummyHeaderName {
        fn try_append(self, map: &mut HeaderMap<String>, value: String) -> Result<bool, MaxSizeReached> {
            map.append(self.0.clone(), value);
            Ok(true)
        }
    }

    let mut map = HeaderMap::with_capacity(10);
    map.try_append(DummyHeaderName("Host".to_string()), "world".to_string()).unwrap();
    
    let result = map.try_append(DummyHeaderName("Host".to_string()), "earth".to_string()).unwrap();
    assert!(result);
    
    let values = map.get_all("host");
    let mut i = values.iter();
    assert_eq!("world", *i.next().unwrap());
    assert_eq!("earth", *i.next().unwrap());
}

#[test]
fn test_try_append_max_capacity() {
    struct DummyHeaderName(String);
    
    impl IntoHeaderName for DummyHeaderName {
        fn try_append(self, map: &mut HeaderMap<String>, _value: String) -> Result<bool, MaxSizeReached> {
            if map.len() >= MAX_SIZE {
                return Err(MaxSizeReached { _priv: () });
            }
            Ok(true)
        }
    }

    let mut map = HeaderMap::try_with_capacity(1).unwrap();
    map.try_append(DummyHeaderName("Host".to_string()), "world".to_string()).unwrap();
    
    let result = map.try_append(DummyHeaderName("AnotherHost".to_string()), "earth".to_string());
    assert!(result.is_err());
}

#[test]
fn test_try_append_empty_key() {
    struct DummyHeaderName;

    impl IntoHeaderName for DummyHeaderName {
        fn try_append(self, _map: &mut HeaderMap<String>, _value: String) -> Result<bool, MaxSizeReached> {
            Ok(false)
        }
    }

    let mut map = HeaderMap::with_capacity(10);
    let result = map.try_append(DummyHeaderName, "empty".to_string()).unwrap();
    assert!(!result);
    assert!(map.is_empty());
}

