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

    fn initialize(&self, value: T) {
        self.cell.set(Some(value));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_value_initialized() {
        let lazy_value = Lazy::new(|| 42);
        lazy_value.initialize(10);
        let result: Result<i32, _> = into_value(lazy_value);
        assert_eq!(result, Ok(10));
    }

    #[test]
    #[should_panic(expected = "Lazy instance has previously been poisoned")]
    fn test_into_value_uninitialized() {
        let lazy_value = Lazy::new(|| 42);
        let result: Result<i32, _> = into_value(lazy_value);
        assert!(result.is_err());
    }
}

