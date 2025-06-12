// Answer 0

#[test]
fn test_into_value_initialized() {
    struct Dummy;
    let lazy = Lazy {
        cell: OnceCell::with_value(42),
        init: Cell::new(Some(Dummy::dummy_init)),
    };
    let result: Result<i32, _> = Lazy::into_value(lazy);
    assert_eq!(result, Ok(42));
}

#[test]
#[should_panic(expected = "Lazy instance has previously been poisoned")]
fn test_into_value_uninitialized() {
    struct Dummy;

    let lazy = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(Some(Dummy::dummy_init)),
    };

    let _ = Lazy::into_value(lazy);
}

#[test]
fn test_into_value_with_custom_init() {
    struct Dummy;

    let lazy = Lazy {
        cell: OnceCell::with_value(84),
        init: Cell::new(Some(Dummy::dummy_init)),
    };

    let result: Result<i32, _> = Lazy::into_value(lazy);
    assert_eq!(result, Ok(84));
}

impl Dummy {
    fn dummy_init() -> Dummy {
        Dummy
    }
}

