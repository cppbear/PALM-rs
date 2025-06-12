// Answer 0

#[test]
#[should_panic(expected = "Lazy instance has previously been poisoned")]
fn test_force_mut_panic_poisoned() {
    use once_cell::sync::Lazy;
    use once_cell::sync::OnceCell;

    struct TestInit {
        initial_value: Option<fn() -> i32>,
    }

    impl TestInit {
        fn new() -> Self {
            TestInit { initial_value: None }
        }
    }

    let mut init = TestInit::new();
    let lazy: &mut Lazy<i32, _> = &mut Lazy::new(|| {
        init.initial_value.take().unwrap()()
    });

    // Simulate poisoning by not providing a function to initialize the value
    init.initial_value = None;

    // This should panic because the Lazy instance has previously been poisoned
    let _result = force_mut(lazy);
}

#[test]
fn test_force_mut_success() {
    use once_cell::sync::Lazy;
    use once_cell::sync::OnceCell;

    let mut lazy = Lazy::new(|| 42);  // Initializes with 42 upon the first call

    let result = force_mut(&mut lazy);
    
    assert_eq!(*result, 42);  // Confirms the value is indeed 42
}

#[test]
fn test_force_mut_no_initialization() {
    use once_cell::sync::Lazy;
    use once_cell::sync::OnceCell;

    struct TestFn {
        func: Option<fn() -> i32>,
    }

    impl TestFn {
        fn new() -> Self {
            TestFn { func: None }
        }
    }

    let mut init = TestFn::new();
    let mut lazy: Lazy<i32, _> = Lazy::new(|| {
        init.func.take().unwrap()()
    });

    // Set it up such that the initialization says there is no function
    init.func = None;

    // This should not panic, the cell is None but there’s no function to call
    // This will also hit the unreachable case hence the panic
    let _result = force_mut(&mut lazy);
}

