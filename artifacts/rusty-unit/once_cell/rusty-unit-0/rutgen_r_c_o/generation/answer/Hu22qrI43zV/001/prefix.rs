// Answer 0

#[test]
fn test_into_value_initialized() {
    let cell = OnceCell::with_value(42);
    let lazy = Lazy::new(|| 43);
    lazy.cell = cell;
    let _ = lazy.into_value();
}

#[test]
fn test_into_value_uninitialized() {
    let cell = OnceCell::new();
    let lazy = Lazy::new(|| 43);
    lazy.cell = cell;
    let result = lazy.into_value();
    let _ = result; // Should be Err, as it is uninitialized
}

#[test]
#[should_panic(expected = "Lazy instance has previously been poisoned")]
fn test_into_value_poisoned() {
    let cell = OnceCell::new();
    let lazy = Lazy::new(|| 43);
    lazy.cell = cell;
    lazy.init.set(Some(|| { panic!("Poisoned state"); }));
    let _ = lazy.into_value();
}

