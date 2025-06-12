// Answer 0

#[test]
fn test_clear_with_existing_extensions() {
    struct Extensions {
        map: Option<std::collections::HashMap<std::any::TypeId, Box<dyn std::any::Any>>>,
    }

    impl Extensions {
        pub fn new() -> Self {
            Extensions { map: Some(std::collections::HashMap::new()) }
        }

        pub fn insert<T: 'static>(&mut self, value: T) {
            if let Some(ref mut map) = self.map {
                map.insert(std::any::TypeId::of::<T>(), Box::new(value));
            }
        }

        pub fn get<T: 'static>(&self) -> Option<&T> {
            self.map.as_ref().and_then(|map| {
                map.get(&std::any::TypeId::of::<T>())
                    .and_then(|boxed| boxed.downcast_ref::<T>())
            })
        }

        pub fn clear(&mut self) {
            if let Some(ref mut map) = self.map {
                map.clear();
            }
        }
    }

    let mut ext = Extensions::new();
    ext.insert(5i32);
    ext.clear();

    assert!(ext.get::<i32>().is_none());
}

#[test]
fn test_clear_with_multiple_extensions() {
    struct Extensions {
        map: Option<std::collections::HashMap<std::any::TypeId, Box<dyn std::any::Any>>>,
    }

    impl Extensions {
        pub fn new() -> Self {
            Extensions { map: Some(std::collections::HashMap::new()) }
        }

        pub fn insert<T: 'static>(&mut self, value: T) {
            if let Some(ref mut map) = self.map {
                map.insert(std::any::TypeId::of::<T>(), Box::new(value));
            }
        }

        pub fn get<T: 'static>(&self) -> Option<&T> {
            self.map.as_ref().and_then(|map| {
                map.get(&std::any::TypeId::of::<T>())
                    .and_then(|boxed| boxed.downcast_ref::<T>())
            })
        }

        pub fn clear(&mut self) {
            if let Some(ref mut map) = self.map {
                map.clear();
            }
        }
    }

    let mut ext = Extensions::new();
    ext.insert(5i32);
    ext.insert("Hello".to_string());
    ext.clear();

    assert!(ext.get::<i32>().is_none());
    assert!(ext.get::<String>().is_none());
}

