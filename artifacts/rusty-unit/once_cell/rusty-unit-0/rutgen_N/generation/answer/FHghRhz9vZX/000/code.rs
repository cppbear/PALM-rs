// Answer 0

#[derive(Debug)]
struct OnceCell<T> {
    inner: std::cell::UnsafeCell<Option<T>>,
}

impl<T> OnceCell<T> {
    pub const fn with_value(value: T) -> OnceCell<T> {
        OnceCell { inner: std::cell::UnsafeCell::new(Some(value)) }
    }
    
    pub fn get(&self) -> Option<&T> {
        unsafe { (*self.inner.get()).as_ref() }
    }
}

#[test]
fn test_with_value_creates_initialized_cell() {
    let cell = OnceCell::with_value(42);
    assert_eq!(cell.get(), Some(&42));
}

#[test]
fn test_with_value_creates_initialized_cell_with_string() {
    let cell = OnceCell::with_value(String::from("Hello"));
    assert_eq!(cell.get(), Some(&String::from("Hello")));
}

#[test]
fn test_with_value_does_not_return_none() {
    let cell = OnceCell::with_value(100);
    assert!(cell.get().is_some());
}

