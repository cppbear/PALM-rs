// Answer 0

#[test]
fn test_get_or_insert_with_new_value() {
    struct Extensions {
        data: std::collections::HashMap<std::any::TypeId, Box<dyn std::any::Any + Send + Sync>>,
    }

    impl Extensions {
        fn new() -> Self {
            Self {
                data: std::collections::HashMap::new(),
            }
        }

        fn get_or_insert_with<T: Clone + Send + Sync + 'static>(
            &mut self,
            value: T,
        ) -> &mut T {
            let type_id = std::any::TypeId::of::<T>();
            self.data
                .entry(type_id)
                .or_insert_with(|| Box::new(value))
                .downcast_mut::<T>()
                .unwrap()
        }

        fn get<T: Send + Sync + 'static>(&self) -> Option<&T> {
            let type_id = std::any::TypeId::of::<T>();
            self.data.get(&type_id)?.downcast_ref::<T>()
        }
    }

    let mut ext = Extensions::new();
    *ext.get_or_insert_with(1i32) += 2;

    assert_eq!(*ext.get::<i32>().unwrap(), 3);
}

#[test]
fn test_get_or_insert_with_existing_value() {
    struct Extensions {
        data: std::collections::HashMap<std::any::TypeId, Box<dyn std::any::Any + Send + Sync>>,
    }

    impl Extensions {
        fn new() -> Self {
            Self {
                data: std::collections::HashMap::new(),
            }
        }

        fn get_or_insert_with<T: Clone + Send + Sync + 'static>(
            &mut self,
            value: T,
        ) -> &mut T {
            let type_id = std::any::TypeId::of::<T>();
            self.data
                .entry(type_id)
                .or_insert_with(|| Box::new(value))
                .downcast_mut::<T>()
                .unwrap()
        }

        fn get<T: Send + Sync + 'static>(&self) -> Option<&T> {
            let type_id = std::any::TypeId::of::<T>();
            self.data.get(&type_id)?.downcast_ref::<T>()
        }
    }

    let mut ext = Extensions::new();
    *ext.get_or_insert_with(2i32) += 3;

    assert_eq!(*ext.get::<i32>().unwrap(), 5);
}

#[test]
fn test_get_or_insert_with_boundary_condition() {
    struct Extensions {
        data: std::collections::HashMap<std::any::TypeId, Box<dyn std::any::Any + Send + Sync>>,
    }

    impl Extensions {
        fn new() -> Self {
            Self {
                data: std::collections::HashMap::new(),
            }
        }

        fn get_or_insert_with<T: Clone + Send + Sync + 'static>(
            &mut self,
            value: T,
        ) -> &mut T {
            let type_id = std::any::TypeId::of::<T>();
            self.data
                .entry(type_id)
                .or_insert_with(|| Box::new(value))
                .downcast_mut::<T>()
                .unwrap()
        }

        fn get<T: Send + Sync + 'static>(&self) -> Option<&T> {
            let type_id = std::any::TypeId::of::<T>();
            self.data.get(&type_id)?.downcast_ref::<T>()
        }
    }

    let mut ext = Extensions::new();
    let value = ext.get_or_insert_with(0u32);
    *value += 1;

    assert_eq!(*ext.get::<u32>().unwrap(), 1);
}

