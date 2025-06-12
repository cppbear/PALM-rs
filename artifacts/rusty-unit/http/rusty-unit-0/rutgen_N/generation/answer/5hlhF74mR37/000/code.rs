// Answer 0

#[derive(Default)]
struct Extensions {
    map: std::collections::HashMap<TypeId, Box<dyn std::any::Any + Send + Sync>>,
}

impl Extensions {
    fn new() -> Self {
        Extensions::default()
    }

    fn get<T: 'static>(&self) -> Option<&T> {
        self.map.get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref::<T>())
    }
}

#[test]
fn test_get_or_insert_with_new_value() {
    let mut ext = Extensions::new();
    *ext.get_or_insert_with(|| 1i32) += 2;

    assert_eq!(*ext.get::<i32>().unwrap(), 3);
}

#[test]
fn test_get_or_insert_with_existing_value() {
    let mut ext = Extensions::new();
    *ext.get_or_insert_with(|| 1i32) += 2;
    *ext.get_or_insert_with(|| 2i32) += 3;

    assert_eq!(*ext.get::<i32>().unwrap(), 5);
}

#[test]
fn test_get_or_insert_with_multiple_types() {
    let mut ext = Extensions::new();
    *ext.get_or_insert_with(|| 1i32) += 2;
    *ext.get_or_insert_with(|| "hello") = "world";

    assert_eq!(*ext.get::<i32>().unwrap(), 3);
    assert_eq!(ext.get::<&str>().unwrap(), &"world");
}

