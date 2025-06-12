// Answer 0

#[derive(Debug)]
struct Lazy<T> {
    value: Option<T>,
}

impl<T> Lazy<T> {
    fn new() -> Self {
        Lazy { value: None }
    }

    fn force_mut(&mut self) -> &mut T {
        if self.value.is_none() {
            panic!("Lazy value has not been initialized.");
        }
        self.value.as_mut().unwrap()
    }

    fn initialize(&mut self, value: T) {
        self.value = Some(value);
    }
}

#[test]
fn test_deref_mut_initialized() {
    let mut lazy = Lazy::new();
    lazy.initialize(42);
    let value: &mut i32 = lazy.force_mut();
    *value += 1;
    assert_eq!(*value, 43);
}

#[test]
#[should_panic(expected = "Lazy value has not been initialized.")]
fn test_deref_mut_not_initialized() {
    let mut lazy = Lazy::new();
    lazy.force_mut();
}

