// Answer 0

#[test]
fn test_remove_existing_extension() {
    struct Extensions {
        map: Option<std::collections::HashMap<std::any::TypeId, Box<dyn std::any::Any + Send + Sync>>>,
    }

    impl Extensions {
        fn new() -> Self {
            Extensions { map: Some(std::collections::HashMap::new()) }
        }

        fn insert<T: Send + Sync + 'static>(&mut self, value: T) {
            if let Some(map) = self.map.as_mut() {
                map.insert(std::any::TypeId::of::<T>(), Box::new(value));
            }
        }

        fn remove<T: Send + Sync + 'static>(&mut self) -> Option<T> {
            self.map
                .as_mut()
                .and_then(|map| map.remove(&std::any::TypeId::of::<T>()))
                .and_then(|boxed| boxed.into_any().downcast().ok().map(|boxed| *boxed))
        }

        fn get<T: Send + Sync + 'static>(&self) -> Option<&T> {
            self.map
                .as_ref()
                .and_then(|map| map.get(&std::any::TypeId::of::<T>()))
                .and_then(|boxed| boxed.downcast_ref::<T>())
        }
    }

    let mut ext = Extensions::new();
    ext.insert(5i32);
    assert_eq!(ext.remove::<i32>(), Some(5i32));
    assert!(ext.get::<i32>().is_none());
}

#[test]
fn test_remove_non_existing_extension() {
    struct Extensions {
        map: Option<std::collections::HashMap<std::any::TypeId, Box<dyn std::any::Any + Send + Sync>>>,
    }

    impl Extensions {
        fn new() -> Self {
            Extensions { map: Some(std::collections::HashMap::new()) }
        }

        fn insert<T: Send + Sync + 'static>(&mut self, value: T) {
            if let Some(map) = self.map.as_mut() {
                map.insert(std::any::TypeId::of::<T>(), Box::new(value));
            }
        }

        fn remove<T: Send + Sync + 'static>(&mut self) -> Option<T> {
            self.map
                .as_mut()
                .and_then(|map| map.remove(&std::any::TypeId::of::<T>()))
                .and_then(|boxed| boxed.into_any().downcast().ok().map(|boxed| *boxed))
        }

        fn get<T: Send + Sync + 'static>(&self) -> Option<&T> {
            self.map
                .as_ref()
                .and_then(|map| map.get(&std::any::TypeId::of::<T>()))
                .and_then(|boxed| boxed.downcast_ref::<T>())
        }
    }

    let mut ext = Extensions::new();
    assert_eq!(ext.remove::<i32>(), None);
}

