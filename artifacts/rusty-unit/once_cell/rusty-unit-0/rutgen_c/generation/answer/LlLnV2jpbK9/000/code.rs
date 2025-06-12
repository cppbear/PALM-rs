// Answer 0

#[test]
fn test_lazy_deref_initializes_value() {
    struct TestLazy {
        value: i32,
    }
    
    let initializer = || {
        TestLazy { value: 42 }
    };

    let lazy = Lazy {
        cell: OnceCell(Imp::new()), 
        init: Cell::new(Some(initializer)),
    };

    let result: &TestLazy = lazy.deref();
    assert_eq!(result.value, 42);
}

#[test]
fn test_lazy_deref_poisoned() {
    struct TestLazy;

    let lazy = Lazy {
        cell: OnceCell(Imp::new()), 
        init: Cell::new(None),
    };

    let result = std::panic::catch_unwind(|| {
        lazy.deref();
    });

    assert!(result.is_err());
}

