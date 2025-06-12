// Answer 0

#[test]
fn test_force_mut_when_cell_is_initialized() {
    use once_cell::sync::{Lazy, OnceCell};

    let value = 42;
    let lazy = Lazy::new(|| value);

    let mut lazy_clone = lazy.clone(); // Clone the lazy to mutate it
    let result = Lazy::force_mut(&mut lazy_clone);

    assert_eq!(*result, value);
}

#[test]
#[should_panic(expected = "Lazy instance has previously been poisoned")]
fn test_force_mut_when_poisoned() {
    use once_cell::sync::{Lazy, OnceCell};
    
    struct PoisonedLazy {
        cell: OnceCell<i32>,
        init: OnceCell<fn() -> i32>,
    }

    impl PoisonedLazy {
        fn new() -> Self {
            let init_value = || panic!("This should panic");
            Self {
                cell: OnceCell::new(),
                init: OnceCell::new().set(init_value).unwrap(),
            }
        }
    }

    let mut lazy = PoisonedLazy::new();

    // First call to force_mut should complete the initialization
    let _ = Lazy::force_mut(&mut lazy); // Should panic here
}

