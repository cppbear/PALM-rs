// Answer 0

#[test]
fn test_lazy_get_uninitialized() {
    use once_cell::unsync::Lazy;

    struct TestType;

    let lazy = Lazy::new(|| TestType);
    assert_eq!(Lazy::get(&lazy), None);
}

#[test]
fn test_lazy_get_initialized() {
    use once_cell::unsync::Lazy;

    let lazy = Lazy::new(|| 42);
    let _ = &*lazy; // Initialize the lazy value
    assert_eq!(Lazy::get(&lazy), Some(&42));
}

#[test]
fn test_lazy_get_multiple_initializations() {
    use once_cell::unsync::Lazy;

    let lazy = Lazy::new(|| 100);
    let _ = &*lazy; // Initialize
    let _ = &*lazy; // Access multiple times, should not panic
    assert_eq!(Lazy::get(&lazy), Some(&100));
}

#[test]
#[should_panic]
fn test_lazy_get_panic_on_double_initialization() {
    use once_cell::unsync::Lazy;
    
    let lazy = Lazy::new(|| panic!("This is a panic!"));
    let _ = &*lazy; // First initialization
    let _ = &*lazy; // Second access should cause panic
}

