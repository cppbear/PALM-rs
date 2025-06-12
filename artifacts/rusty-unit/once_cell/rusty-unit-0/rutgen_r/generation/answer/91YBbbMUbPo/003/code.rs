// Answer 0

#[test]
fn test_force_mut_when_cell_is_some() {
    use once_cell::unsync::{Lazy, OnceCell};

    struct TestFunc;
    
    impl TestFunc {
        fn new() -> Self {
            TestFunc
        }
    }

    let mut lazy = Lazy::new(|| 42); // Initialization that returns 42
    let _ = Lazy::force_mut(&mut lazy); // Forces evaluation, should initialize
    let result = Lazy::force_mut(&mut lazy); // Should not panic, and return a mutable reference

    assert_eq!(*result, 42); // Evaluating the result should still yield 42
    assert!(lazy.cell.get_mut().is_some()); // Ensure that cell is no longer empty
}

#[test]
#[should_panic]
fn test_force_mut_panics_if_poisoned() {
    use once_cell::unsync::{Lazy, OnceCell};

    struct PoisonedFunc;
    
    impl PoisonedFunc {
        fn new() -> Self {
            PoisonedFunc
        }
    }

    let mut lazy = Lazy::new(|| panic!("Lazy instance has previously been poisoned"));

    // First call to force_mut will panic
    let _ = Lazy::force_mut(&mut lazy); 
}

