// Answer 0

#[derive(Debug)]
struct Lazy<T> {
    value: Option<T>,
}

impl<T> Lazy<T> {
    fn new() -> Self {
        Lazy { value: None }
    }

    fn force(&self) -> &T {
        self.value.as_ref().expect("Value is not initialized")
    }

    fn initialize(&mut self, value: T) {
        self.value = Some(value);
    }
}

#[test]
fn test_deref_initialized() {
    let mut lazy_value = Lazy::new();
    lazy_value.initialize(42);
    let value: &i32 = lazy_value.force();
    assert_eq!(*value, 42);
}

#[test]
#[should_panic(expected = "Value is not initialized")]
fn test_deref_uninitialized() {
    let lazy_value: Lazy<i32> = Lazy::new();
    let _value: &i32 = lazy_value.force();
}

