// Answer 0

#[test]
fn test_extend_with_different_types() {
    struct Extensions {
        map: Option<std::collections::HashMap<String, Box<dyn std::any::Any>>>,
    }

    impl Extensions {
        fn new() -> Self {
            Extensions {
                map: Some(std::collections::HashMap::new()),
            }
        }

        fn insert<T: 'static>(&mut self, value: T) {
            if let Some(map) = &mut self.map {
                map.insert(std::any::type_name::<T>().to_string(), Box::new(value));
            }
        }

        fn get<T: 'static>(&self) -> Option<&T> {
            self.map.as_ref()?.get(std::any::type_name::<T>())
                .and_then(|boxed| boxed.downcast_ref::<T>())
        }

        fn len(&self) -> usize {
            self.map.as_ref().map_or(0, |m| m.len())
        }

        fn extend(&mut self, other: Self) {
            if let Some(other_map) = other.map {
                if let Some(self_map) = &mut self.map {
                    self_map.extend(other_map);
                } else {
                    self.map = Some(other_map);
                }
            }
        }
    }

    let mut ext_a = Extensions::new();
    ext_a.insert(8u8);
    ext_a.insert(16u16);
    
    let mut ext_b = Extensions::new();
    ext_b.insert(4u8);
    ext_b.insert("hello");

    ext_a.extend(ext_b);
    
    assert_eq!(ext_a.len(), 3);
    assert_eq!(ext_a.get::<u8>(), Some(&4u8));
    assert_eq!(ext_a.get::<u16>(), Some(&16u16));
    assert_eq!(ext_a.get::<&'static str>().copied(), Some("hello"));
}

#[test]
fn test_extend_with_same_type() {
    struct Extensions {
        map: Option<std::collections::HashMap<String, Box<dyn std::any::Any>>>,
    }

    impl Extensions {
        fn new() -> Self {
            Extensions {
                map: Some(std::collections::HashMap::new()),
            }
        }

        fn insert<T: 'static>(&mut self, value: T) {
            if let Some(map) = &mut self.map {
                map.insert(std::any::type_name::<T>().to_string(), Box::new(value));
            }
        }

        fn get<T: 'static>(&self) -> Option<&T> {
            self.map.as_ref()?.get(std::any::type_name::<T>())
                .and_then(|boxed| boxed.downcast_ref::<T>())
        }

        fn len(&self) -> usize {
            self.map.as_ref().map_or(0, |m| m.len())
        }

        fn extend(&mut self, other: Self) {
            if let Some(other_map) = other.map {
                if let Some(self_map) = &mut self.map {
                    self_map.extend(other_map);
                } else {
                    self.map = Some(other_map);
                }
            }
        }
    }

    let mut ext_a = Extensions::new();
    ext_a.insert(8u8);

    let mut ext_b = Extensions::new();
    ext_b.insert(4u8);

    ext_a.extend(ext_b);
    
    assert_eq!(ext_a.len(), 1);
    assert_eq!(ext_a.get::<u8>(), Some(&4u8));
}

