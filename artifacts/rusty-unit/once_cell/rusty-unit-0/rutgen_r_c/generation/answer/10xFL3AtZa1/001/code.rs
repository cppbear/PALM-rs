// Answer 0

#[test]
fn test_lazy_into_value_initialized() {
    let once_cell = OnceCell::with_value("Initialized");
    let lazy = Lazy {
        cell: once_cell,
        init: Cell::new(None),
    };
    assert_eq!(lazy.into_value().unwrap(), "Initialized");
}

#[test]
#[should_panic(expected = "Lazy instance has previously been poisoned")]
fn test_lazy_into_value_not_initialized() {
    let once_cell = OnceCell::new();
    let lazy = Lazy {
        cell: once_cell,
        init: Cell::new(Some(|| "Initializer")),
    };
    lazy.into_value().unwrap_err();
}

