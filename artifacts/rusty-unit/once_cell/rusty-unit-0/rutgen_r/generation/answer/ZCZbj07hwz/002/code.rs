// Answer 0

#[test]
#[should_panic(expected = "Lazy instance has previously been poisoned")]
fn test_force_mut_panic_on_poisoning() {
    struct TestLazy;
    impl TestLazy {
        fn new<F>(f: F) -> Lazy<i32, F>
        where
            F: FnOnce() -> i32,
        {
            Lazy {
                cell: OnceCell::new(),
                init: OnceCell::new().with_value(f),
            }
        }
    }

    let mut lazy: Lazy<i32, fn() -> i32> = TestLazy::new(|| panic!("This should not run"));
    let mut lazy_ref = &mut lazy;
    drop(lazy_ref.init.take()); // Simulate poisoning
    
    let _ = force_mut(lazy_ref);
}

#[test]
fn test_force_mut_initialized_value() {
    struct TestLazy;
    impl TestLazy {
        fn new<F>(f: F) -> Lazy<i32, F>
        where
            F: FnOnce() -> i32,
        {
            Lazy {
                cell: OnceCell::new(),
                init: OnceCell::new().with_value(f),
            }
        }
    }

    let mut lazy: Lazy<i32, fn() -> i32> = TestLazy::new(|| 42);
    let mut lazy_ref = &mut lazy;

    let result = force_mut(lazy_ref);
    
    assert_eq!(*result, 42);
}

#[test]
fn test_force_mut_twice() {
    struct TestLazy;
    impl TestLazy {
        fn new<F>(f: F) -> Lazy<i32, F>
        where
            F: FnOnce() -> i32,
        {
            Lazy {
                cell: OnceCell::new(),
                init: OnceCell::new().with_value(f),
            }
        }
    }

    let mut lazy: Lazy<i32, fn() -> i32> = TestLazy::new(|| 99);
    let mut lazy_ref = &mut lazy;

    let first_result = force_mut(lazy_ref);
    assert_eq!(*first_result, 99);

    let second_result = force_mut(lazy_ref);
    assert_eq!(*second_result, 99);
}

