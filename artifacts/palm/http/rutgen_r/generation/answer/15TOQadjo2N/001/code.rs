// Answer 0

#[test]
fn test_get_or_insert_with_new_value() {
    struct Extensions {
        data: std::collections::HashMap<std::any::TypeId, Box<dyn std::any::Any + Send + Sync>>,
    }

    impl Extensions {
        fn new() -> Self {
            Extensions {
                data: std::collections::HashMap::new(),
            }
        }

        fn get_or_insert<T: Clone + Send + Sync + 'static>(&mut self, value: T) -> &mut T {
            self.get_or_insert_with(|| value)
        }

        fn get_or_insert_with<T: Clone + Send + Sync + 'static>(&mut self, factory: impl FnOnce() -> T) -> &mut T {
            let type_id = std::any::TypeId::of::<T>();
            if let Some(entry) = self.data.get_mut(&type_id) {
                entry.downcast_mut::<T>().unwrap()
            } else {
                let value = factory();
                let value_boxed = Box::new(value);
                self.data.insert(type_id, value_boxed);
                self.data.get_mut(&type_id).unwrap().downcast_mut::<T>().unwrap()
            }
        }

        fn get<T: std::any::Any>(&self) -> Option<&T> {
            let type_id = std::any::TypeId::of::<T>();
            self.data.get(&type_id)?.downcast_ref::<T>()
        }
    }

    let mut ext = Extensions::new();
    *ext.get_or_insert(1i32) += 2;
    
    assert_eq!(*ext.get::<i32>().unwrap(), 3);
}

#[test]
fn test_get_or_insert_with_existing_value() {
    struct Extensions {
        data: std::collections::HashMap<std::any::TypeId, Box<dyn std::any::Any + Send + Sync>>,
    }

    impl Extensions {
        fn new() -> Self {
            Extensions {
                data: std::collections::HashMap::new(),
            }
        }

        fn get_or_insert<T: Clone + Send + Sync + 'static>(&mut self, value: T) -> &mut T {
            self.get_or_insert_with(|| value)
        }

        fn get_or_insert_with<T: Clone + Send + Sync + 'static>(&mut self, factory: impl FnOnce() -> T) -> &mut T {
            let type_id = std::any::TypeId::of::<T>();
            if let Some(entry) = self.data.get_mut(&type_id) {
                entry.downcast_mut::<T>().unwrap()
            } else {
                let value = factory();
                let value_boxed = Box::new(value);
                self.data.insert(type_id, value_boxed);
                self.data.get_mut(&type_id).unwrap().downcast_mut::<T>().unwrap()
            }
        }

        fn get<T: std::any::Any>(&self) -> Option<&T> {
            let type_id = std::any::TypeId::of::<T>();
            self.data.get(&type_id)?.downcast_ref::<T>()
        }
    }

    let mut ext = Extensions::new();
    *ext.get_or_insert(5i32) += 2; // Set value to 5
    *ext.get_or_insert(10i32) += 2; // Should not insert, modify existing

    assert_eq!(*ext.get::<i32>().unwrap(), 7);
}

