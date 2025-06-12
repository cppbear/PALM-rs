// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::{Any, TypeId};
    use std::collections::HashMap;
    
    struct Extensions {
        map: Option<HashMap<TypeId, Box<dyn Any + Send + Sync>>>,
    }

    impl Extensions {
        pub fn new() -> Self {
            Extensions { map: Some(HashMap::new()) }
        }

        pub fn insert<T: Send + Sync + 'static>(&mut self, value: T) {
            if let Some(ref mut map) = self.map {
                map.insert(TypeId::of::<T>(), Box::new(value));
            }
        }
        
        pub fn get<T: Send + Sync + 'static>(&self) -> Option<&T> {
            self.map
                .as_ref()
                .and_then(|map| map.get(&TypeId::of::<T>()))
                .and_then(|boxed| (**boxed).as_any().downcast_ref())
        }
    }

    #[test]
    fn test_get_none_before_insertion() {
        let ext = Extensions::new();
        assert!(ext.get::<i32>().is_none());
    }

    #[test]
    fn test_get_some_after_insertion() {
        let mut ext = Extensions::new();
        ext.insert(5i32);
        assert_eq!(ext.get::<i32>(), Some(&5i32));
    }

    #[test]
    fn test_get_different_types() {
        let mut ext = Extensions::new();
        ext.insert(10i32);
        ext.insert("Hello".to_string());

        assert_eq!(ext.get::<i32>(), Some(&10i32));
        assert_eq!(ext.get::<String>(), Some(&"Hello".to_string()));
        assert!(ext.get::<f64>().is_none());
    }

    #[test]
    fn test_get_multiple_insertions() {
        let mut ext = Extensions::new();
        ext.insert(1i32);
        ext.insert(2i32);
        ext.insert(3i32);

        assert_eq!(ext.get::<i32>(), Some(&3i32)); // The last inserted value
    }
}

