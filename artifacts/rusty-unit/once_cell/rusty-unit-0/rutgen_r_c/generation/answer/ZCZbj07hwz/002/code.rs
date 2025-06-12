// Answer 0

#[test]
fn test_force_mut_initializes_value() {
    struct TestLazy {
        cell: OnceCell<i32>,
        init: Cell<Option<fn() -> i32>>,
    }

    impl TestLazy {
        fn new(f: fn() -> i32) -> Self {
            TestLazy {
                cell: OnceCell::new(),
                init: Cell::new(Some(f)),
            }
        }
    }

    let mut lazy = TestLazy::new(|| 42);
    let result: &mut i32 = Lazy::force_mut(&mut lazy);
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
        fn new() -> Self {
            TestLazy {
                cell: OnceCell::new(),
                init: Cell::new(None),
            }
        }
    }

    let mut lazy = TestLazy::new();
    Lazy::force_mut(&mut lazy);
}

