// Answer 0

#[test]
fn test_get_uninitialized() {
    use once_cell::sync::Lazy;
    
    let lazy = Lazy::new(|| 92);
    
    assert_eq!(Lazy::get(&lazy), None);
}

#[test]
fn test_get_initialized() {
    use once_cell::sync::Lazy;
    
    let lazy = Lazy::new(|| 92);
    let _value = &*lazy; // Initializes the lazy value
    
    assert_eq!(Lazy::get(&lazy), Some(&92));
}

