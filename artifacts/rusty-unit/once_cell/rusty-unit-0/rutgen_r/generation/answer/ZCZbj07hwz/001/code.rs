// Answer 0

#[test]
fn test_force_mut_initializes_and_returns_mutable_reference() {
    use once_cell::sync::{Lazy, OnceCell};

    struct MyLazy {
        cell: OnceCell<i32>,
        init: OnceCell<Box<dyn FnOnce() -> i32>>,
    }

    let mut lazy = MyLazy {
        cell: OnceCell::new(),
        init: OnceCell::with_value(Box::new(|| 42)),
    };

    let result = force_mut(&mut lazy);
    
    // Assert that the value is initialized correctly
    assert_eq!(*result, 42);
}

#[test]
#[should_panic(expected = "Lazy instance has previously been poisoned")]
fn test_force_mut_panics_when_poisoned() {
    use once_cell::sync::{Lazy, OnceCell};

    struct MyLazy {
        cell: OnceCell<i32>,
        init: OnceCell<Box<dyn FnOnce() -> i32>>,
    }

    let mut lazy = MyLazy {
        cell: OnceCell::new(),
        init: OnceCell::new(), // This mimics a poisoned Lazy as it contains no initialization function.
    };

    let _result = force_mut(&mut lazy);
}

#[test]
fn test_force_mut_multiple_initialization() {
    use once_cell::sync::{Lazy, OnceCell};

    struct MyLazy {
        cell: OnceCell<i32>,
        init: OnceCell<Box<dyn FnOnce() -> i32>>,
    }

    let mut lazy = MyLazy {
        cell: OnceCell::new(),
        init: OnceCell::with_value(Box::new(|| 100)),
    };

    let result1 = force_mut(&mut lazy);
    
    // Initialize again and ensure it does not panic after the first initialization
    let result2 = force_mut(&mut lazy);

    assert_eq!(*result1, 100);
    assert_eq!(result1 as *const _ as usize, result2 as *const _ as usize); // Check that they point to the same memory
}

