// Answer 0

#[test]
fn test_lazy_deref_with_initialized_value() {
    struct InitValue;

    let init_value = || InitValue;
    let lazy = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(Some(init_value)),
    };

    let result: &InitValue = lazy.deref();
    assert!(result.is_initialized());
}

#[test]
#[should_panic(expected = "Lazy instance has previously been poisoned")]
fn test_lazy_deref_with_poisoned_instance() {
    struct InitValue;

    let init_value = || InitValue;
    let mut lazy = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(Some(init_value)),
    };

    // Force initialization to consume the closure
    let _ = lazy.deref();

    // Now poison it by not resetting the init cell
    lazy.init.set(None);

    // This should panic
    let _ = lazy.deref();
} 

#[test]
fn test_lazy_deref_after_initialization() {
    struct InitValue(u32);

    let init_value = || InitValue(42);
    let mut lazy = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(Some(init_value)),
    };

    let _ = lazy.deref(); // Initialize

    // Now dereference again after initialization
    let result: &InitValue = lazy.deref();
    assert_eq!(result.0, 42);
}

#[test]
fn test_lazy_deref_with_no_initialization() {
    struct InitValue;

    let lazy = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(None),
    };

    // Should panic since there is no initialization closure
    let result = std::panic::catch_unwind(|| {
        let _ = lazy.deref();
    });
    assert!(result.is_err());
}

