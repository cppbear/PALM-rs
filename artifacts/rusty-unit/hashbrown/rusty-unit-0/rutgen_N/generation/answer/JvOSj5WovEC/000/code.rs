// Answer 0

#[derive(Debug)]
struct ScopeGuard<T> {
    value: T,
}

impl<T> ScopeGuard<T> {
    fn new(value: T) -> Self {
        ScopeGuard { value }
    }

    fn deref(&self) -> &T {
        &self.value
    }
}

#[test]
fn test_deref() {
    let guard = ScopeGuard::new(42);
    assert_eq!(*guard.deref(), 42);
}

#[test]
fn test_deref_with_string() {
    let guard = ScopeGuard::new(String::from("Hello"));
    assert_eq!(guard.deref(), &String::from("Hello"));
}

#[test]
fn test_deref_with_empty_string() {
    let guard = ScopeGuard::new(String::from(""));
    assert_eq!(guard.deref(), &String::from(""));
}

