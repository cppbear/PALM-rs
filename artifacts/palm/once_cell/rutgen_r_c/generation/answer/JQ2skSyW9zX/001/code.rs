// Answer 0

#[test]
fn test_lazy_get_uninitialized() {
    struct ExampleLazy;
    let lazy = Lazy::<i32, fn() -> i32>::new(|| 42);
    
    assert_eq!(Lazy::get(&lazy), None);
}

#[test]
fn test_lazy_get_initialized() {
    struct ExampleLazy;
    let mut lazy = Lazy::new(|| 42);
    
    // Force initialization
    Lazy::force(&lazy);
    
    assert_eq!(Lazy::get(&lazy), Some(&42));
}

#[test]
fn test_lazy_get_return_value() {
    struct ExampleLazy;
    let lazy = Lazy::new(|| 100);
    
    let value = Lazy::get(&lazy);
    assert!(value.is_none()); // Initially uninitialized.

    // Now force it to initialize.
    let initialized_value = Lazy::force(&lazy);
    
    assert_eq!(initialized_value, &100); // Should be initialized now.
    assert_eq!(Lazy::get(&lazy), Some(&100)); // Should return Some(&100) now.
}

