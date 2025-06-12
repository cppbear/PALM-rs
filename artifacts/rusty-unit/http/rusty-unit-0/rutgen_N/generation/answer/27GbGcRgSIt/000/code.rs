// Answer 0

#[test]
fn test_insert_new_integer() {
    use std::any::{Any, TypeId};
    use std::collections::HashMap;

    struct Extensions {
        map: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
    }

    impl Extensions {
        pub fn new() -> Self {
            Extensions {
                map: HashMap::new(),
            }
        }

        pub fn insert<T: Clone + Send + Sync + 'static>(&mut self, val: T) -> Option<T> {
            self.map
                .get_or_insert_with(Box::default)
                .insert(TypeId::of::<T>(), Box::new(val))
                .and_then(|boxed| boxed.downcast::<T>().ok().map(|boxed| *boxed))
        }
    }

    let mut ext = Extensions::new();
    assert!(ext.insert(5i32).is_none());
}

#[test]
fn test_insert_new_u8() {
    use std::any::{Any, TypeId};
    use std::collections::HashMap;

    struct Extensions {
        map: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
    }

    impl Extensions {
        pub fn new() -> Self {
            Extensions {
                map: HashMap::new(),
            }
        }

        pub fn insert<T: Clone + Send + Sync + 'static>(&mut self, val: T) -> Option<T> {
            self.map
                .get_or_insert_with(Box::default)
                .insert(TypeId::of::<T>(), Box::new(val))
                .and_then(|boxed| boxed.downcast::<T>().ok().map(|boxed| *boxed))
        }
    }

    let mut ext = Extensions::new();
    assert!(ext.insert(4u8).is_none());
}

#[test]
fn test_insert_replace_integer() {
    use std::any::{Any, TypeId};
    use std::collections::HashMap;

    struct Extensions {
        map: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
    }

    impl Extensions {
        pub fn new() -> Self {
            Extensions {
                map: HashMap::new(),
            }
        }

        pub fn insert<T: Clone + Send + Sync + 'static>(&mut self, val: T) -> Option<T> {
            self.map
                .get_or_insert_with(Box::default)
                .insert(TypeId::of::<T>(), Box::new(val))
                .and_then(|boxed| boxed.downcast::<T>().ok().map(|boxed| *boxed))
        }
    }

    let mut ext = Extensions::new();
    assert!(ext.insert(5i32).is_none());
    assert_eq!(ext.insert(9i32), Some(5i32));
}

