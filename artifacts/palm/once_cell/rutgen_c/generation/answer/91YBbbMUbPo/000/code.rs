// Answer 0

#[test]
fn test_force_mut_initialization() {
    struct TestInit;
    
    impl TestInit {
        fn new() -> fn() -> i32 {
            || 42
        }
    }

    let mut lazy = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(Some(TestInit::new())),
    };
    
    let value = Lazy::force_mut(&mut lazy);
    assert_eq!(*value, 42);
}

#[test]
fn test_force_mut_with_poisoned() {
    struct TestInit;
    
    impl TestInit {
        fn new() -> fn() -> i32 {
            || 42
        }
    }

    let mut lazy = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(None),
    };

    std::panic::catch_unwind(|| {
        Lazy::force_mut(&mut lazy);
    }).unwrap_err();
}

