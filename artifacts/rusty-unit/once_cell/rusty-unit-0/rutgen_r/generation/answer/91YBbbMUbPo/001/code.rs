// Answer 0

#[test]
fn test_force_mut_initializes_value() {
    struct TestLazy {
        cell: once_cell::unsync::OnceCell<i32>,
        init: std::option::Option<Box<dyn FnMut() -> i32>>,
    }

    let mut lazy = TestLazy {
        cell: once_cell::unsync::OnceCell::new(),
        init: Some(Box::new(|| 42)),
    };

    let result = force_mut(&mut lazy);
    assert_eq!(*result, 42);
    assert_eq!(lazy.cell.get_mut().unwrap(), &42);
}

#[test]
#[should_panic(expected = "Lazy instance has previously been poisoned")]
fn test_force_mut_panics_when_poisoned() {
    struct TestLazy {
        cell: once_cell::unsync::OnceCell<i32>,
        init: std::option::Option<Box<dyn FnMut() -> i32>>,
    }

    let mut lazy = TestLazy {
        cell: once_cell::unsync::OnceCell::new(),
        init: None, // This simulates the poisoned state
    };

    force_mut(&mut lazy);
}

