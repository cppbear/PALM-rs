// Answer 0

#[test]
fn test_extend_with_overlapping_and_non_overlapping_types() {
    struct Extensions {
        map: Option<std::collections::HashMap<std::any::TypeId, Box<dyn std::any::Any>>>,
    }

    impl Extensions {
        fn new() -> Self {
            Extensions { map: Some(std::collections::HashMap::new()) }
        }

        fn insert<T: 'static>(&mut self, value: T) {
            if let Some(map) = &mut self.map {
                map.insert(std::any::TypeId::of::<T>(), Box::new(value));
            }
        }

        fn get<T: 'static>(&self) -> Option<&T> {
            self.map.as_ref()?.get(&std::any::TypeId::of::<T>())
                .and_then(|value| value.downcast_ref::<T>())
        }

        fn len(&self) -> usize {
            self.map.as_ref().map_or(0, |map| map.len())
        }

        fn extend(&mut self, other: Self) {
            if let Some(other_map) = other.map {
                if let Some(map) = &mut self.map {
                    map.extend(other_map);
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
fn test_extend_with_empty_other() {
    struct Extensions {
        map: Option<std::collections::HashMap<std::any::TypeId, Box<dyn std::any::Any>>>,
    }

    impl Extensions {
        fn new() -> Self {
            Extensions { map: Some(std::collections::HashMap::new()) }
        }

        fn insert<T: 'static>(&mut self, value: T) {
            if let Some(map) = &mut self.map {
                map.insert(std::any::TypeId::of::<T>(), Box::new(value));
            }
        }

        fn len(&self) -> usize {
            self.map.as_ref().map_or(0, |map| map.len())
        }

        fn extend(&mut self, other: Self) {
            if let Some(other_map) = other.map {
                if let Some(map) = &mut self.map {
                    map.extend(other_map);
                } else {
                    self.map = Some(other_map);
                }
            }
        }
    }

    let mut ext_a = Extensions::new();
    ext_a.insert(8u8);

    let ext_b = Extensions::new();

    ext_a.extend(ext_b);
    assert_eq!(ext_a.len(), 1);
    assert_eq!(ext_a.get::<u8>(), Some(&8u8));
}

#[test]
fn test_extend_with_same_type_different_values() {
    struct Extensions {
        map: Option<std::collections::HashMap<std::any::TypeId, Box<dyn std::any::Any>>>,
    }

    impl Extensions {
        fn new() -> Self {
            Extensions { map: Some(std::collections::HashMap::new()) }
        }

        fn insert<T: 'static>(&mut self, value: T) {
            if let Some(map) = &mut self.map {
                map.insert(std::any::TypeId::of::<T>(), Box::new(value));
            }
        }

        fn get<T: 'static>(&self) -> Option<&T> {
            self.map.as_ref()?.get(&std::any::TypeId::of::<T>())
                .and_then(|value| value.downcast_ref::<T>())
        }

        fn len(&self) -> usize {
            self.map.as_ref().map_or(0, |map| map.len())
        }

        fn extend(&mut self, other: Self) {
            if let Some(other_map) = other.map {
                if let Some(map) = &mut self.map {
                    map.extend(other_map);
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

