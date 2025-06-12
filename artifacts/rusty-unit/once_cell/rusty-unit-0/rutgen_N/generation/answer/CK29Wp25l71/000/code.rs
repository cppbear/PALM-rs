// Answer 0

#[test]
fn test_get_or_init_with_initialization() {
    struct Cell {
        inner: once_cell::sync::Lazy<bool>,
    }

    impl Cell {
        fn new() -> Self {
            Cell {
                inner: once_cell::sync::Lazy::new(|| false),
            }
        }

        fn get_or_init<F>(&self, f: F) -> bool
        where
            F: FnOnce() -> bool,
        {
            *self.inner.get_or_init(|| f())
        }
    }

    let cell = Cell::new();
    let initialized_value = cell.get_or_init(|| true);
    assert_eq!(initialized_value, true);
    let reinitialized_value = cell.get_or_init(|| false);
    assert_eq!(reinitialized_value, true);
}

#[test]
fn test_get_or_init_with_false_initialization() {
    struct Cell {
        inner: once_cell::sync::Lazy<bool>,
    }

    impl Cell {
        fn new() -> Self {
            Cell {
                inner: once_cell::sync::Lazy::new(|| false),
            }
        }

        fn get_or_init<F>(&self, f: F) -> bool
        where
            F: FnOnce() -> bool,
        {
            *self.inner.get_or_init(|| f())
        }
    }

    let cell = Cell::new();
    let initialized_value = cell.get_or_init(|| false);
    assert_eq!(initialized_value, false);
    let reinitialized_value = cell.get_or_init(|| true);
    assert_eq!(reinitialized_value, false);
}

