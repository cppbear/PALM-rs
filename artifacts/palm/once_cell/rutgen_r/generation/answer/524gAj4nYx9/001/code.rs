// Answer 0

#[derive(Debug)]
struct SafeCell<T>(Option<T>);

impl<T> SafeCell<T> {
    fn new() -> Self {
        SafeCell(None)
    }

    fn initialize(&mut self, value: T) {
        self.0 = Some(value);
    }

    unsafe fn get_unchecked(&self) -> &T {
        self.0.as_ref().unwrap() // This may panic if not initialized
    }
}

#[test]
fn test_get_unchecked_initialized() {
    let mut cell = SafeCell::new();
    cell.initialize(42);
    unsafe {
        let value: &i32 = cell.get_unchecked();
        assert_eq!(*value, 42);
    }
}

#[test]
#[should_panic]
fn test_get_unchecked_uninitialized() {
    let cell = SafeCell::new();
    unsafe {
        let _value: &i32 = cell.get_unchecked(); // This should panic
    }
}

