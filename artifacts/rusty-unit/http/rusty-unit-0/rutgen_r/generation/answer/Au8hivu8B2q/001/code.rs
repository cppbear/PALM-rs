// Answer 0

#[test]
fn test_get_mut_with_existing_type() {
    use std::any::{Any, TypeId};
    use std::collections::HashMap;

    struct Extensions {
        map: Option<HashMap<TypeId, Box<dyn Any + Send + Sync>>>,
    }

    impl Extensions {
        fn new() -> Self {
            Extensions { map: Some(HashMap::new()) }
        }
        
        fn insert<T: Any + Send + Sync>(&mut self, value: T) {
            if let Some(map) = self.map.as_mut() {
                map.insert(TypeId::of::<T>(), Box::new(value));
            }
        }
        
        pub fn get_mut<T: Send + Sync + 'static>(&mut self) -> Option<&mut T> {
            self.map
                .as_mut()
                .and_then(|map| map.get_mut(&TypeId::of::<T>()))
                .and_then(|boxed| (**boxed).as_any_mut().downcast_mut())
        }
    }

    let mut ext = Extensions::new();
    ext.insert(String::from("Hello"));

    let value = ext.get_mut::<String>().unwrap();
    value.push_str(" World");

    assert_eq!(ext.get_mut::<String>().unwrap(), "Hello World");
}

#[test]
fn test_get_mut_with_non_existent_type() {
    use std::any::{Any, TypeId};
    use std::collections::HashMap;

    struct Extensions {
        map: Option<HashMap<TypeId, Box<dyn Any + Send + Sync>>>,
    }

    impl Extensions {
        fn new() -> Self {
            Extensions { map: Some(HashMap::new()) }
        }
        
        fn insert<T: Any + Send + Sync>(&mut self, value: T) {
            if let Some(map) = self.map.as_mut() {
                map.insert(TypeId::of::<T>(), Box::new(value));
            }
        }
        
        pub fn get_mut<T: Send + Sync + 'static>(&mut self) -> Option<&mut T> {
            self.map
                .as_mut()
                .and_then(|map| map.get_mut(&TypeId::of::<T>()))
                .and_then(|boxed| (**boxed).as_any_mut().downcast_mut())
        }
    }

    let mut ext = Extensions::new();
    ext.insert(String::from("Hello"));

    let result: Option<&mut i32> = ext.get_mut::<i32>();
    assert!(result.is_none());
}

#[test]
#[should_panic]
fn test_get_mut_panic_on_empty_map() {
    use std::any::{Any, TypeId};
    use std::collections::HashMap;

    struct Extensions {
        map: Option<HashMap<TypeId, Box<dyn Any + Send + Sync>>>,
    }

    impl Extensions {
        fn new() -> Self {
            Extensions { map: Some(HashMap::new()) }
        }
        
        fn insert<T: Any + Send + Sync>(&mut self, value: T) {
            if let Some(map) = self.map.as_mut() {
                map.insert(TypeId::of::<T>(), Box::new(value));
            }
        }
        
        pub fn get_mut<T: Send + Sync + 'static>(&mut self) -> Option<&mut T> {
            self.map
                .as_mut()
                .and_then(|map| map.get_mut(&TypeId::of::<T>()))
                .and_then(|boxed| (**boxed).as_any_mut().downcast_mut())
        }
    }

    let mut ext: Extensions = Extensions::new();
    ext.get_mut::<String>().unwrap();
}

