// Answer 0

#[test]
fn test_append_preserve_order() {
    #[derive(Default)]
    struct MapImpl {
        inner: std::collections::HashMap<String, String>,
    }

    #[derive(Default)]
    struct MyMap {
        map: MapImpl,
    }

    impl MyMap {
        pub fn append(&mut self, other: &mut Self) {
            #[cfg(feature = "preserve_order")]
            self.map
                .inner
                .extend(std::mem::replace(&mut other.map.inner, MapImpl::default()));
            #[cfg(not(feature = "preserve_order"))]
            self.map.inner.extend(other.map.inner.drain());
        }
    }

    let mut map1 = MyMap {
        map: MapImpl {
            inner: {
                let mut m = std::collections::HashMap::new();
                m.insert("key1".to_string(), "value1".to_string());
                m.insert("key2".to_string(), "value2".to_string());
                m
            }
        },
    };
    let mut map2 = MyMap {
        map: MapImpl {
            inner: {
                let mut m = std::collections::HashMap::new();
                m.insert("key3".to_string(), "value3".to_string());
                m.insert("key4".to_string(), "value4".to_string());
                m
            }
        },
    };

    map1.append(&mut map2);

    assert_eq!(map1.map.inner.len(), 4);
    assert_eq!(map1.map.inner.get("key1"), Some(&"value1".to_string()));
    assert_eq!(map1.map.inner.get("key3"), Some(&"value3".to_string()));
    assert!(map2.map.inner.is_empty());
}

#[test]
fn test_append_non_empty_to_empty() {
    #[derive(Default)]
    struct MapImpl {
        inner: std::collections::HashMap<String, String>,
    }

    #[derive(Default)]
    struct MyMap {
        map: MapImpl,
    }

    impl MyMap {
        pub fn append(&mut self, other: &mut Self) {
            #[cfg(feature = "preserve_order")]
            self.map
                .inner
                .extend(std::mem::replace(&mut other.map.inner, MapImpl::default()));
            #[cfg(not(feature = "preserve_order"))]
            self.map.inner.extend(other.map.inner.drain());
        }
    }

    let mut map1 = MyMap::default(); // empty map
    let mut map2 = MyMap {
        map: MapImpl {
            inner: {
                let mut m = std::collections::HashMap::new();
                m.insert("key1".to_string(), "value1".to_string());
                m.insert("key2".to_string(), "value2".to_string());
                m
            }
        },
    };

    map1.append(&mut map2);

    assert_eq!(map1.map.inner.len(), 2);
    assert_eq!(map1.map.inner.get("key1"), Some(&"value1".to_string()));
    assert_eq!(map1.map.inner.get("key2"), Some(&"value2".to_string()));
    assert!(map2.map.inner.is_empty());
}

#[test]
fn test_append_empty_to_non_empty() {
    #[derive(Default)]
    struct MapImpl {
        inner: std::collections::HashMap<String, String>,
    }

    #[derive(Default)]
    struct MyMap {
        map: MapImpl,
    }

    impl MyMap {
        pub fn append(&mut self, other: &mut Self) {
            #[cfg(feature = "preserve_order")]
            self.map
                .inner
                .extend(std::mem::replace(&mut other.map.inner, MapImpl::default()));
            #[cfg(not(feature = "preserve_order"))]
            self.map.inner.extend(other.map.inner.drain());
        }
    }

    let mut map1 = MyMap {
        map: MapImpl {
            inner: {
                let mut m = std::collections::HashMap::new();
                m.insert("key1".to_string(), "value1".to_string());
                m
            }
        },
    }; 
    let mut map2 = MyMap::default(); // empty map

    map1.append(&mut map2);

    assert_eq!(map1.map.inner.len(), 1);
    assert_eq!(map1.map.inner.get("key1"), Some(&"value1".to_string()));
    assert!(map2.map.inner.is_empty());
}

