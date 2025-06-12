// Answer 0

#[test]
fn test_lazy_get_none_before_initialization() {
    use once_cell::unsync::Lazy;

    let lazy: Lazy<i32, fn() -> i32> = Lazy::new(|| 92);

    assert_eq!(Lazy::get(&lazy), None);
}

#[test]
fn test_lazy_get_after_initialization() {
    use once_cell::unsync::Lazy;

    let lazy: Lazy<i32, fn() -> i32> = Lazy::new(|| 92);

    let _value = &*lazy; // This initializes the Lazy instance.
    
    assert_eq!(Lazy::get(&lazy), Some(&92));
}

#[test]
fn test_lazy_get_multiple_access() {
    use once_cell::unsync::Lazy;

    let lazy: Lazy<String, fn() -> String> = Lazy::new(|| "Hello".to_string());

    assert_eq!(Lazy::get(&lazy), None);
    let _value = &*lazy; // Trigger initialization
    assert_eq!(Lazy::get(&lazy), Some(&"Hello".to_string()));
}

#[test]
fn test_lazy_get_no_side_effects() {
    use once_cell::unsync::Lazy;

    let call_count = std::cell::Cell::new(0);
    let lazy: Lazy<i32, fn() -> i32> = Lazy::new(|| {
        call_count.set(call_count.get() + 1);
        42
    });

    assert_eq!(Lazy::get(&lazy), None);
    assert_eq!(call_count.get(), 0); // Should not have been called yet
    let _value = &*lazy; // Initialize
    assert_eq!(Lazy::get(&lazy), Some(&42));
    assert_eq!(call_count.get(), 1); // Should be called only once
}

