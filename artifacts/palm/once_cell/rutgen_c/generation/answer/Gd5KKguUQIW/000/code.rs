// Answer 0

#[test]
fn test_deref_mut_initialization() {
    use core::cell::Cell;

    struct Dummy;
    let initializer: fn() -> Dummy = || Dummy;
    
    let mut lazy_instance = Lazy {
        cell: OnceCell(Imp::new()),
        init: Cell::new(Some(initializer)),
    };

    let value: &mut Dummy = lazy_instance.deref_mut();
    assert!(value.is_ptr());
}

#[test]
#[should_panic(expected = "Lazy instance has previously been poisoned")]
fn test_deref_mut_poisoned() {
    use core::cell::Cell;

    struct Dummy;
    let initializer: fn() -> Dummy = || Dummy;

    let mut lazy_instance = Lazy {
        cell: OnceCell(Imp::new()),
        init: Cell::new(None),
    };

    lazy_instance.deref_mut();
}

#[test]
fn test_deref_mut_after_initialization() {
    use core::cell::Cell;

    struct Dummy;
    let initializer: fn() -> Dummy = || Dummy;

    let mut lazy_instance = Lazy {
        cell: OnceCell(Imp::new()),
        init: Cell::new(Some(initializer)),
    };

    let _value: &mut Dummy = lazy_instance.deref_mut();
    
    let value: &mut Dummy = lazy_instance.deref_mut();  // Calling again to ensure it does not panic
    assert!(value.is_ptr());
}

