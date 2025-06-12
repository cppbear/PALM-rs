// Answer 0

#[test]
fn test_append_preserve_order() {
    #[cfg(feature = "preserve_order")]
    {
        use std::collections::HashMap;

        struct MapImpl {
            map: HashMap<String, i32>,
        }

        impl Default for MapImpl {
            fn default() -> Self {
                Self {
                    map: HashMap::new(),
                }
            }
        }

        let mut a = MapImpl::default();
        a.map.insert("key1".to_string(), 1);
        a.map.insert("key2".to_string(), 2);

        let mut b = MapImpl::default();
        b.map.insert("key3".to_string(), 3);
        
        a.append(&mut b);

        assert_eq!(a.map.len(), 3);
        assert!(a.map.contains_key("key1"));
        assert!(a.map.contains_key("key2"));
        assert!(a.map.contains_key("key3"));
        assert_eq!(b.map.len(), 0);
    }
}

#[test]
fn test_append_no_preserve_order() {
    #[cfg(not(feature = "preserve_order"))]
    {
        use std::collections::VecDeque;

        struct MapImpl {
            map: VecDeque<(String, i32)>,
        }

        impl Default for MapImpl {
            fn default() -> Self {
                Self {
                    map: VecDeque::new(),
                }
            }
        }

        impl MapImpl {
            fn append(&mut self, other: &mut Self) {
                self.map.append(&mut other.map);
            }
        }

        let mut a = MapImpl::default();
        a.map.push_back(("key1".to_string(), 1));
        a.map.push_back(("key2".to_string(), 2));

        let mut b = MapImpl::default();
        b.map.push_back(("key3".to_string(), 3));
        
        a.append(&mut b);

        assert_eq!(a.map.len(), 3);
        assert_eq!(b.map.len(), 0);
    }
}

