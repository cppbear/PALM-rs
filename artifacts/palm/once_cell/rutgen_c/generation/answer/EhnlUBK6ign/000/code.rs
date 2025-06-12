// Answer 0

#[cfg(test)]
fn test_force() {
    struct TestLazy<'a> {
        cell: OnceCell<i32>,
        init: Cell<Option<fn() -> i32>>,
    }

    impl<'a> Lazy<i32, fn() -> i32> {
        fn new(init: fn() -> i32) -> Self {
            Self {
                cell: OnceCell(UnsafeCell::new(None)),
                init: Cell::new(Some(init)),
            }
        }
    }

    let lazy = TestLazy::new(|| 92);
    assert_eq!(Lazy::force(&lazy), &92);
}

#[cfg(test)]
fn test_force_multiple_calls() {
    struct TestLazy<'a> {
        cell: OnceCell<i32>,
        init: Cell<Option<fn() -> i32>>,
    }

    impl<'a> Lazy<i32, fn() -> i32> {
        fn new(init: fn() -> i32) -> Self {
            Self {
                cell: OnceCell(UnsafeCell::new(None)),
                init: Cell::new(Some(init)),
            }
        }
    }

    let lazy = TestLazy::new(|| 42);
    assert_eq!(Lazy::force(&lazy), &42);
    assert_eq!(Lazy::force(&lazy), &42); // Check that further calls are consistent.
}

#[cfg(test)]
#[should_panic(expected = "Lazy instance has previously been poisoned")]
fn test_force_poisoning() {
    struct TestLazy<'a> {
        cell: OnceCell<i32>,
        init: Cell<Option<fn() -> i32>>,
    }

    impl<'a> Lazy<i32, fn() -> i32> {
        fn new(init: fn() -> i32) -> Self {
            Self {
                cell: OnceCell(UnsafeCell::new(None)),
                init: Cell::new(Some(init)),
            }
        }
    }

    let mut lazy = TestLazy::new(|| panic!("Initialization panic")); 
    Lazy::force(&lazy); // This call should panic during initialization.
}

#[cfg(test)]
fn test_force_with_custom_data() {
    struct TestLazy<'a> {
        cell: OnceCell<String>,
        init: Cell<Option<fn() -> String>>,
    }

    impl<'a> Lazy<String, fn() -> String> {
        fn new(init: fn() -> String) -> Self {
            Self {
                cell: OnceCell(UnsafeCell::new(None)),
                init: Cell::new(Some(init)),
            }
        }
    }

    let lazy = TestLazy::new(|| "Hello, World!".to_string());
    assert_eq!(Lazy::force(&lazy), "Hello, World!"); 
}

