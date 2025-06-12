// Answer 0

#[test]
fn test_lazy_initialization_with_function() {
    use once_cell::unsync::Lazy;
    
    let lazy = Lazy::new(|| "Hello, World!".to_uppercase());
    assert_eq!(&*lazy, "HELLO, WORLD!");
}

#[test]
fn test_lazy_initialization_with_empty_string() {
    use once_cell::unsync::Lazy;
    
    let lazy = Lazy::new(|| "".to_string());
    assert_eq!(&*lazy, "");
}

#[test]
fn test_lazy_initialization_with_numeric_conversion() {
    use once_cell::unsync::Lazy;
    
    let lazy = Lazy::new(|| (42).to_string());
    assert_eq!(&*lazy, "42");
}

#[test]
fn test_lazy_initialization_with_complex_expression() {
    use once_cell::unsync::Lazy;
    
    let text = "Rust".to_string();
    let lazy = Lazy::new(|| format!("{} is awesome!", text));
    assert_eq!(&*lazy, "Rust is awesome!");
}

#[should_panic]
fn test_lazy_initialization_with_panic() {
    use once_cell::unsync::Lazy;
    
    let lazy = Lazy::new(|| panic!("This should panic!"));
    let _ = &*lazy; // This access will trigger the panic
}

