// Answer 0

#[test]
fn test_force_mut_with_none_cell_and_none_init() {
    struct Lazy<T, F> {
        cell: OnceCell<T>,
        init: Option<F>,
    }

    struct OnceCell<T> {
        value: Option<T>,
    }

    impl<T> OnceCell<T> {
        fn new() -> Self {
            OnceCell { value: None }
        }

        fn with_value(value: T) -> Self {
            OnceCell { value: Some(value) }
        }

        fn get_mut(&mut self) -> Option<&mut T> {
            self.value.as_mut()
        }
    }

    impl<T> Lazy<T, ()> {
        fn new<Fn>(_f: Fn) -> Self {
            Lazy {
                cell: OnceCell::new(),
                init: None,
            }
        }
    }

    let mut lazy: Lazy<i32, ()> = Lazy::new(|| 92);
    let result = std::panic::catch_unwind(|| {
        force_mut(&mut lazy);
    });
    
    assert!(result.is_err());
}

#[test]
#[should_panic(expected = "Lazy instance has previously been poisoned")]
fn test_force_mut_with_none_cell_and_poisoned_init() {
    struct Lazy<T, F> {
        cell: OnceCell<T>,
        init: Option<F>,
    }

    struct OnceCell<T> {
        value: Option<T>,
    }

    impl<T> OnceCell<T> {
        fn new() -> Self {
            OnceCell { value: None }
        }

        fn with_value(value: T) -> Self {
            OnceCell { value: Some(value) }
        }

        fn get_mut(&mut self) -> Option<&mut T> {
            self.value.as_mut()
        }
    }

    impl<T> Lazy<T, ()> {
        fn new<Fn>(_f: Fn) -> Self {
            Lazy {
                cell: OnceCell::new(),
                init: None,
            }
        }
    }

    let mut lazy: Lazy<i32, ()> = Lazy::new(|| 92);
    
    // Simulate poisoned state by setting init to None
    lazy.init = None;

    let result = std::panic::catch_unwind(|| {
        force_mut(&mut lazy);
    });
    
    assert!(result.is_err());
}

