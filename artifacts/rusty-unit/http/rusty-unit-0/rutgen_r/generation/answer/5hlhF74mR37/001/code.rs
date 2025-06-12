// Answer 0

#[test]
fn test_get_or_insert_with_new_value() {
    use std::any::{Any, TypeId};
    use std::collections::HashMap;

    struct Extensions {
        map: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
    }

    impl Extensions {
        pub fn new() -> Self {
            Extensions { map: HashMap::new() }
        }

        pub fn get_or_insert_with<T: Clone + Send + Sync + 'static, F: FnOnce() -> T>(
            &mut self,
            f: F,
        ) -> &mut T {
            let out = self.map.entry(TypeId::of::<T>())
                .or_insert_with(|| Box::new(f()));
            (**out).as_any_mut().downcast_mut().unwrap()
        }
        
        pub fn get<T: 'static>(&self) -> Option<&T> {
            self.map.get(&TypeId::of::<T>())
                .and_then(|boxed| boxed.downcast_ref::<T>())
        }
    }

    let mut ext = Extensions::new();
    *ext.get_or_insert_with(|| 1i32) += 2;

    assert_eq!(*ext.get::<i32>().unwrap(), 3);
}

#[test]
fn test_get_or_insert_with_existing_value() {
    use std::any::{Any, TypeId};
    use std::collections::HashMap;

    struct Extensions {
        map: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
    }

    impl Extensions {
        pub fn new() -> Self {
            Extensions { map: HashMap::new() }
        }

        pub fn get_or_insert_with<T: Clone + Send + Sync + 'static, F: FnOnce() -> T>(
            &mut self,
            f: F,
        ) -> &mut T {
            let out = self.map.entry(TypeId::of::<T>())
                .or_insert_with(|| Box::new(f()));
            (**out).as_any_mut().downcast_mut().unwrap()
        }

        pub fn get<T: 'static>(&self) -> Option<&T> {
            self.map.get(&TypeId::of::<T>())
                .and_then(|boxed| boxed.downcast_ref::<T>())
        }
    }

    let mut ext = Extensions::new();
    ext.get_or_insert_with(|| 5i32);

    *ext.get_or_insert_with(|| 1i32) += 2; // Should not change the existing value

    assert_eq!(*ext.get::<i32>().unwrap(), 5);
}

#[test]
#[should_panic]
fn test_get_or_insert_with_wrong_type() {
    use std::any::{Any, TypeId};
    use std::collections::HashMap;

    struct Extensions {
        map: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
    }

    impl Extensions {
        pub fn new() -> Self {
            Extensions { map: HashMap::new() }
        }

        pub fn get_or_insert_with<T: Clone + Send + Sync + 'static, F: FnOnce() -> T>(
            &mut self,
            f: F,
        ) -> &mut T {
            let out = self.map.entry(TypeId::of::<T>())
                .or_insert_with(|| Box::new(f()));
            (**out).as_any_mut().downcast_mut().unwrap()
        }

        pub fn get<T: 'static>(&self) -> Option<&T> {
            self.map.get(&TypeId::of::<T>())
                .and_then(|boxed| boxed.downcast_ref::<T>())
        }
    }

    let mut ext = Extensions::new();
    ext.get_or_insert_with(|| 3.14f64); // Insert a f64

    // Attempt to insert a value of type i32 and then retrieve it by wrong type.
    let _: &i32 = ext.get_or_insert_with(|| 1i32);
}

