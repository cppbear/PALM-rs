// Answer 0

#[test]
fn test_try_insert_new_key() {
    use http::HeaderMap;
    use http::header::HOST;
    
    let mut map = HeaderMap::new();
    assert!(map.try_insert(HOST, "world".parse().unwrap()).unwrap().is_none());
    assert!(!map.is_empty());
}

#[test]
fn test_try_insert_existing_key() {
    use http::HeaderMap;
    use http::header::HOST;
    
    let mut map = HeaderMap::new();
    assert!(map.try_insert(HOST, "world".parse().unwrap()).unwrap().is_none());
    
    let prev = map.try_insert(HOST, "earth".parse().unwrap()).unwrap().unwrap();
    assert_eq!("world", prev);
}

#[test]
fn test_try_insert_exceed_capacity() {
    use http::HeaderMap;
    use http::header::HOST;
    use std::collections::HashMap;
    
    struct LimitedHeaderMap {
        map: HeaderMap,
        max_size: usize,
    }

    impl LimitedHeaderMap {
        fn new(max_size: usize) -> Self {
            LimitedHeaderMap {
                map: HeaderMap::new(),
                max_size,
            }
        }

        fn try_insert(&mut self, key: &str, value: &str) -> Result<Option<String>, &'static str> {
            if self.map.len() < self.max_size {
                self.map.try_insert(key.parse().unwrap(), value.parse().unwrap()).map_err(|_| "Max size reached")
            } else {
                Err("Max size reached")
            }
        }
    }

    let mut map = LimitedHeaderMap::new(1);
    assert!(map.try_insert("HOST", "localhost").is_ok());
    assert_eq!(map.try_insert("HOST", "127.0.0.1").unwrap_err(), "Max size reached");
}

