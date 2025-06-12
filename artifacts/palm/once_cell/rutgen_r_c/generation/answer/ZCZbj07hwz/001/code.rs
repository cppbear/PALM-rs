// Answer 0

#[test]
fn test_force_mut_initializes_cell() {
    use core::cell::Cell;

    struct TestLazy {
        cell: OnceCell<i32>,
        init: Cell<Option<fn() -> i32>>,
    }

    impl TestLazy {
        fn new(init_fn: fn() -> i32) -> Self {
            TestLazy {
                cell: OnceCell::new(),
                init: Cell::new(Some(init_fn)),
            }
        }
    }

    let mut lazy = TestLazy::new(|| 42);
    let result = Lazy::<i32>::force_mut(&mut lazy);
    assert_eq!(*result, 42);
}

#[test]
#[should_panic(expected = "Lazy instance has previously been poisoned")]
fn test_force_mut_panics_when_init_is_none() {
    struct TestLazy {
        cell: OnceCell<i32>,
        init: Cell<Option<fn() -> i32>>,
    }

    impl TestLazy {
        fn new(init_fn: Option<fn() -> i32>) -> Self {
            TestLazy {
                cell: OnceCell::new(),
                init: Cell::new(init_fn),
            }
        }
    }

    let mut lazy = TestLazy::new(None);
    let _result = Lazy::<i32>::force_mut(&mut lazy);
}

#[test]
fn test_force_mut_multiple_calls() {
    struct TestLazy {
        cell: OnceCell<i32>,
        init: Cell<Option<fn() -> i32>>,
    }

    impl TestLazy {
        fn new(init_fn: fn() -> i32) -> Self {
            TestLazy {
                cell: OnceCell::new(),
                init: Cell::new(Some(init_fn)),
            }
        }
    }

    let mut lazy = TestLazy::new(|| 100);
    let first_result = Lazy::<i32>::force_mut(&mut lazy);
    assert_eq!(*first_result, 100);
    
    let second_result = Lazy::<i32>::force_mut(&mut lazy);
    assert_eq!(first_result as *mut _, second_result as *mut _); // Ensure same reference is returned
}

