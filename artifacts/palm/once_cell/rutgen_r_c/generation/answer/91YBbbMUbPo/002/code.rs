// Answer 0

#[test]
fn test_force_mut_with_initialized_value() {
    let mut lazy = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(Some(|| 42)),
    };

    let value = Lazy::force_mut(&mut lazy);
    assert_eq!(*value, 42);
}

#[test]
#[should_panic(expected = "Lazy instance has previously been poisoned")]
fn test_force_mut_with_poisoned_lazy() {
    let mut lazy = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(None),
    };

    Lazy::force_mut(&mut lazy);
}

#[test]
fn test_force_mut_with_empty_once_cell() {
    let mut lazy = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(Some(|| 100)),
    };

    let value = Lazy::force_mut(&mut lazy);
    assert_eq!(*value, 100);
}

