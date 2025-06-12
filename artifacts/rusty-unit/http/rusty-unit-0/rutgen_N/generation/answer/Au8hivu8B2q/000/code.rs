// Answer 0

#[test]
fn test_get_mut_existing_type() {
    use std::any::TypeId;
    use std::collections::HashMap;
    use std::sync::Arc;

    struct Extensions {
        map: Option<HashMap<TypeId, Arc<dyn std::any::Any + Send + Sync>>>,
    }

    impl Extensions {
        pub fn new() -> Self {
            Extensions { map: Some(HashMap::new()) }
        }

        pub fn insert<T: Send + Sync + 'static>(&mut self, value: T) {
            if let Some(map) = self.map.as_mut() {
                map.insert(TypeId::of::<T>(), Arc::new(value));
            }
        }

        pub fn get_mut<T: Send + Sync + 'static>(&mut self) -> Option<&mut T> {
            self.map
                .as_mut()
                .and_then(|map| map.get_mut(&TypeId::of::<T>()))
                .and_then(|boxed| (**boxed).as_any_mut().downcast_mut())
        }
        
        pub fn get<T: Send + Sync + 'static>(&self) -> Option<&T> {
            self.map
                .as_ref()
                .and_then(|map| map.get(&TypeId::of::<T>()))
                .and_then(|boxed| boxed.as_any().downcast_ref())
        }
    }

    let mut ext = Extensions::new();
    ext.insert(String::from("Hello"));
    ext.get_mut::<String>().unwrap().push_str(" World");

    assert_eq!(ext.get::<String>().unwrap(), "Hello World");
}

#[test]
fn test_get_mut_non_existent_type() {
    use std::any::TypeId;
    use std::collections::HashMap;
    use std::sync::Arc;

    struct Extensions {
        map: Option<HashMap<TypeId, Arc<dyn std::any::Any + Send + Sync>>>,
    }

    impl Extensions {
        pub fn new() -> Self {
            Extensions { map: Some(HashMap::new()) }
        }

        pub fn insert<T: Send + Sync + 'static>(&mut self, value: T) {
            if let Some(map) = self.map.as_mut() {
                map.insert(TypeId::of::<T>(), Arc::new(value));
            }
        }

        pub fn get_mut<T: Send + Sync + 'static>(&mut self) -> Option<&mut T> {
            self.map
                .as_mut()
                .and_then(|map| map.get_mut(&TypeId::of::<T>()))
                .and_then(|boxed| (**boxed).as_any_mut().downcast_mut())
        }
        
        pub fn get<T: Send + Sync + 'static>(&self) -> Option<&T> {
            self.map
                .as_ref()
                .and_then(|map| map.get(&TypeId::of::<T>()))
                .and_then(|boxed| boxed.as_any().downcast_ref())
        }
    }

    let mut ext = Extensions::new();
    ext.insert(String::from("Hello"));
    
    let result: Option<&mut u32> = ext.get_mut::<u32>();
    assert!(result.is_none());
}

