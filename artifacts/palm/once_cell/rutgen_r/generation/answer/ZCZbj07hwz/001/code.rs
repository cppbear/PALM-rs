// Answer 0

#[test]
fn test_force_mut_initializes_and_returns_value() {
    use once_cell::sync::{Lazy, OnceCell};

    struct MyStruct;

    let mut init_value = Some(|| 42);
    let mut lazy = Lazy {
        init: OnceCell::with_value(&mut init_value),
        cell: OnceCell::new(),
    };

    let result: &mut i32 = force_mut(&mut lazy);
    
    assert_eq!(*result, 42);
}

#[test]
#[should_panic(expected = "Lazy instance has previously been poisoned")]
fn test_force_mut_panics_on_poisoning() {
    use once_cell::sync::{Lazy, OnceCell};

    let mut lazy = Lazy {
        init: OnceCell::new(),
        cell: OnceCell::new(),
    };
    
    // Simulate poisoning by not initializing `init`
    force_mut(&mut lazy);
}

#[test]
fn test_force_mut_with_different_function() {
    use once_cell::sync::{Lazy, OnceCell};

    struct MyStruct;

    let mut init_value = Some(|| 99);
    let mut lazy = Lazy {
        init: OnceCell::with_value(&mut init_value),
        cell: OnceCell::new(),
    };

    let result: &mut i32 = force_mut(&mut lazy);
    
    assert_eq!(*result, 99);
}

