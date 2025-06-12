// Answer 0

#[test]
fn test_or_try_insert_with_occupied() {
    struct TestHeaderMap {
        map: std::collections::HashMap<String, String>,
    }

    impl TestHeaderMap {
        fn new() -> Self {
            TestHeaderMap {
                map: std::collections::HashMap::new(),
            }
        }

        fn try_insert(&mut self, key: &str, value: String) -> Result<(), ()> {
            self.map.insert(key.to_string(), value);
            Ok(())
        }

        fn try_entry(&mut self, key: &str) -> Option<&mut String> {
            self.map.get_mut(key).map(|v| v)
        }

        fn or_try_insert_with<F>(&mut self, key: &str, default: F) -> Result<&mut String, ()>
        where
            F: FnOnce() -> String,
        {
            match self.try_entry(key) {
                Some(value) => Ok(value),
                None => {
                    let value = default();
                    self.try_insert(key, value.clone()).unwrap();
                    Ok(self.try_entry(key).unwrap())
                }
            }
        }
    }

    let mut map = TestHeaderMap::new();
    map.try_insert("x-hello", "original value".to_string()).unwrap();

    let res = map.or_try_insert_with("x-hello", || "default value".to_string()).unwrap();
    assert_eq!(res, &"original value".to_string());
}

#[test]
fn test_or_try_insert_with_vacant() {
    struct TestHeaderMap {
        map: std::collections::HashMap<String, String>,
    }

    impl TestHeaderMap {
        fn new() -> Self {
            TestHeaderMap {
                map: std::collections::HashMap::new(),
            }
        }

        fn try_insert(&mut self, key: &str, value: String) -> Result<(), ()> {
            self.map.insert(key.to_string(), value);
            Ok(())
        }

        fn try_entry(&mut self, key: &str) -> Option<&mut String> {
            self.map.get_mut(key).map(|v| v)
        }

        fn or_try_insert_with<F>(&mut self, key: &str, default: F) -> Result<&mut String, ()>
        where
            F: FnOnce() -> String,
        {
            match self.try_entry(key) {
                Some(value) => Ok(value),
                None => {
                    let value = default();
                    self.try_insert(key, value.clone()).unwrap();
                    Ok(self.try_entry(key).unwrap())
                }
            }
        }
    }

    let mut map = TestHeaderMap::new();

    let res = map.or_try_insert_with("new-key", || "new-value".to_string()).unwrap();
    assert_eq!(res, &"new-value".to_string());
}

