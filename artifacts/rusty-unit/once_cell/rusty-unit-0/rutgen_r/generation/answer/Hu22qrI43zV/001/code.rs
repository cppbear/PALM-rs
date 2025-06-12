// Answer 0

#[derive(Debug)]
struct Lazy<T, F> {
    cell: Cell<Option<T>>,
    init: Option<F>,
}

impl<T, F> Lazy<T, F> {
    fn new(init: F) -> Self {
        Lazy {
            cell: Cell::new(None),
            init: Some(init),
        }
    }

    fn set_value(&self, value: T) {
        self.cell.set(Some(value));
    }
}

#[test]
fn test_into_value_initialized() {
    let init_value = || "initialization";
    let lazy = Lazy::new(init_value);
    let test_value = "test_value";
    lazy.set_value(test_value);
    
    let result: Result<&str, &fn()> = into_value(lazy);
    assert_eq!(result, Ok(test_value));
}

#[test]
#[should_panic(expected = "Lazy instance has previously been poisoned")]
fn test_into_value_not_initialized_panics() {
    let init_value = || "initialization";
    let lazy = Lazy::new(init_value);
    
    let _: Result<&str, &fn()> = into_value(lazy);
}

#[test]
fn test_into_value_not_initialized_returns_err() {
    let init_value = || "initialization";
    let lazy = Lazy::new(init_value);
    
    let result: Result<&str, &fn()> = into_value(lazy);
    assert!(result.is_err());
}

