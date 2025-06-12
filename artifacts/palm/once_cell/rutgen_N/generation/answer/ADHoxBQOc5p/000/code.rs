// Answer 0

#[test]
fn test_lazy_initialization() {
    use once_cell::Lazy;
    use std::cell::OnceCell;

    let lazy_value: Lazy<i32, fn() -> i32> = Lazy::new(|| 42);
    assert_eq!(lazy_value.get(), None);
    let value = lazy_value.get_or_init();
    assert_eq!(*value, 42);
    assert_eq!(lazy_value.get(), Some(&42));
}

#[test]
fn test_lazy_multiple_initializations() {
    use once_cell::Lazy;
    use std::cell::OnceCell;

    static LAZY: Lazy<i32, fn() -> i32> = Lazy::new(|| {
        assert!(LAZY.get().is_none()); // Should be initialized the first time it's called
        100
    });

    assert_eq!(*LAZY.get_or_init(), 100);
    assert_eq!(*LAZY.get(), Some(&100));
} 

#[test]
#[should_panic]
fn test_lazy_panic_initialization() {
    use once_cell::Lazy;
    use std::cell::OnceCell;

    let _: Lazy<i32, fn() -> i32> = Lazy::new(|| panic!("Initialization failed!"));
    let _ = Lazy::<i32, fn() -> i32>::get_or_init();
}

