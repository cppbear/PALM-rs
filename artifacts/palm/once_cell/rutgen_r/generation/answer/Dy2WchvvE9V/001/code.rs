// Answer 0

#[test]
fn test_lazy_force_with_initialization() {
    struct Lazy<T, F> {
        cell: Cell<Option<T>>,
        init: Option<F>,
    }

    impl<T, F> Lazy<T, F> {
        fn new(init: F) -> Self {
            Self {
                cell: Cell::new(None),
                init: Some(init),
            }
        }
    }

    struct Cell<T> {
        value: Option<T>,
    }

    impl<T> Cell<T> {
        fn new(value: Option<T>) -> Self {
            Self { value }
        }

        fn get_or_init<F>(&self, init: F) -> &T 
        where
            F: FnOnce() -> T,
        {
            if self.value.is_none() {
                self.value = Some(init());
            }
            self.value.as_ref().unwrap()
        }
    }

    let lazy = Lazy::new(|| 92);
    assert_eq!(force(&lazy), &92);
}

#[test]
#[should_panic(expected = "Lazy instance has previously been poisoned")]
fn test_lazy_force_with_poisoned_instance() {
    struct Lazy<T, F> {
        cell: Cell<Option<T>>,
        init: Option<F>,
    }

    impl<T, F> Lazy<T, F> {
        fn new(init: F) -> Self {
            Self {
                cell: Cell::new(None),
                init: Some(init),
            }
        }
    }

    struct Cell<T> {
        value: Option<T>,
    }

    impl<T> Cell<T> {
        fn new(value: Option<T>) -> Self {
            Self { value }
        }

        fn get_or_init<F>(&self, init: F) -> &T
        where
            F: FnOnce() -> T,
        {
            if self.value.is_none() {
                self.value = Some(init());
            }
            self.value.as_ref().unwrap()
        }
    }

    let mut lazy = Lazy::new(|| 92);
    // Simulating poison by dropping the initializer
    lazy.init = None;
    force(&lazy);
}

