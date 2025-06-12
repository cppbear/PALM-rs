// Answer 0

#[test]
fn test_lazy_force_with_valid_initialization() {
    struct TestLazy;

    impl TestLazy {
        fn new() -> Lazy<i32> {
            Lazy {
                cell: OnceCell::new(),
                init: Cell::new(Some(|| 42)),
            }
        }
    }

    let lazy = TestLazy::new();
    let result = Lazy::force(&lazy);
    assert_eq!(*result, 42);
}

#[test]
#[should_panic(expected = "Lazy instance has previously been poisoned")]
fn test_lazy_force_with_poisoned_lazy() {
    struct PoisonedLazy;

    impl PoisonedLazy {
        fn new() -> Lazy<i32> {
            Lazy {
                cell: OnceCell::new(),
                init: Cell::new(None), // Simulate poison
            }
        }
    }

    let lazy = PoisonedLazy::new();
    Lazy::force(&lazy); // This should panic
}

