// Answer 0

#[test]
fn test_lazy_new_with_valid_function() {
    let hello = "Hello, World!".to_string();
    let lazy = Lazy::new(|| hello.to_uppercase());

    assert_eq!(&*lazy, "HELLO, WORLD!");
}

#[test]
fn test_lazy_new_with_empty_initialization() {
    let lazy: Lazy<(), fn() -> ()> = Lazy::new(|| {});

    assert_eq!(&*lazy, &());
}

#[test]
fn test_lazy_new_with_panic_initialization() {
    let result = std::panic::catch_unwind(|| {
        let _lazy: Lazy<(), fn() -> ()> = Lazy::new(|| panic!("Panicking!"));
    });

    assert!(result.is_err());
}

