// Answer 0

#[test]
fn test_lazy_initialization() {
    use once_cell::unsync::Lazy;

    let hello = "Hello, World!".to_string();
    let lazy = Lazy::new(|| hello.to_uppercase());

    assert_eq!(&*lazy, "HELLO, WORLD!");
}

#[test]
fn test_lazy_reinitialization() {
    use once_cell::unsync::Lazy;

    let hello = "Hello, World!".to_string();
    let lazy = Lazy::new(|| hello.to_uppercase());

    assert_eq!(&*lazy, "HELLO, WORLD!");

    // Ensure it doesn't recompute
    let original_reference = &*lazy;
    assert_eq!(original_reference, &*lazy);
}

#[test]
fn test_lazy_with_empty_string() {
    use once_cell::unsync::Lazy;

    let lazy = Lazy::new(|| "".to_string());

    assert_eq!(&*lazy, "");
}

#[test]
fn test_lazy_with_numeric_string() {
    use once_cell::unsync::Lazy;

    let lazy = Lazy::new(|| "12345".to_string());

    assert_eq!(&*lazy, "12345");
}

#[test]
#[should_panic]
fn test_lazy_with_panic() {
    use once_cell::unsync::Lazy;

    let lazy = Lazy::new(|| {
        panic!("This should panic");
    });

    let _ = &*lazy; // access to trigger the panic
}

