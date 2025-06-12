// Answer 0

#[derive(Debug)]
struct Lazy<T> {
    value: Option<T>,
}

impl<T> Lazy<T> {
    fn force_mut(&mut self) -> &mut T {
        if self.value.is_none() {
            self.value = Some(Default::default());
        }
        self.value.as_mut().unwrap()
    }
}

#[test]
fn test_deref_mut_initialization() {
    let mut lazy: Lazy<i32> = Lazy { value: None };
    let value_ref = lazy.deref_mut();
    *value_ref = 42; // Initialize the value
    assert_eq!(*value_ref, 42);
}

#[test]
fn test_deref_mut_existing_value() {
    let mut lazy: Lazy<String> = Lazy { value: Some("Hello".to_string()) };
    let value_ref = lazy.deref_mut();
    value_ref.push_str(", World!");
    assert_eq!(*value_ref, "Hello, World!");
}

#[test]
fn test_deref_mut_multiple_calls() {
    let mut lazy: Lazy<f64> = Lazy { value: None };
    let value_ref1 = lazy.deref_mut();
    *value_ref1 += 3.14;
    let value_ref2 = lazy.deref_mut();
    *value_ref2 *= 2.0;
    assert_eq!(*value_ref2, 6.28);
}

