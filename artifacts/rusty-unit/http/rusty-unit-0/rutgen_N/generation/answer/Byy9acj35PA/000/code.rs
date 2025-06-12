// Answer 0

#[test]
fn test_clear_extensions() {
    struct Extensions {
        map: Option<std::collections::HashMap<std::any::TypeId, Box<dyn std::any::Any>>>,
    }

    impl Extensions {
        fn new() -> Self {
            Extensions { map: Some(std::collections::HashMap::new()) }
        }

        fn insert<T: 'static>(&mut self, value: T) {
            if let Some(ref mut map) = self.map {
                map.insert(std::any::TypeId::of::<T>(), Box::new(value));
            }
        }

        fn get<T: 'static>(&self) -> Option<&T> {
            self.map.as_ref()?.get(&std::any::TypeId::of::<T>()).and_then(|v| v.downcast_ref::<T>())
        }

        fn clear(&mut self) {
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

