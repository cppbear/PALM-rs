// Answer 0

#[test]
fn test_get_mut_with_uninitialized_lazy() {
    struct TestLazy;
    
    impl TestLazy {
        fn new() -> Lazy<i32, fn() -> i32> {
            Lazy {
                cell: OnceCell::new(),
                init: Cell::new(None),
            }
        }
    }

    let mut lazy = TestLazy::new();
    assert_eq!(Lazy::get_mut(&mut lazy), None);
}

#[test]
fn test_get_mut_with_initialized_lazy() {
    struct TestLazy;

    impl TestLazy {
        fn new() -> Lazy<i32, fn() -> i32> {
            Lazy {
                cell: OnceCell::with_value(42),
                init: Cell::new(Some(|| 42)),
            }
        }
    }

    let mut lazy = TestLazy::new();
    assert!(Lazy::get_mut(&mut lazy).is_some());
    let value = Lazy::get_mut(&mut lazy).unwrap();
    *value += 1; // mutate the value
    assert_eq!(*value, 43);
}

#[test]
fn test_get_mut_after_force_initialization() {
    struct TestLazy;

    impl TestLazy {
        fn new() -> Lazy<i32, fn() -> i32> {
            Lazy {
                cell: OnceCell::new(),
                init: Cell::new(Some(|| 100)),
            }
        }
    }

    let mut lazy = TestLazy::new();
    Lazy::force(&lazy); // force initialization
    assert!(Lazy::get_mut(&mut lazy).is_some());
    let value = Lazy::get_mut(&mut lazy).unwrap();
    *value += 10; // mutate the value
    assert_eq!(*value, 110);
}

