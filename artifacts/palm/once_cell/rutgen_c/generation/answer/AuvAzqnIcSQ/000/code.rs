// Answer 0

#[test]
fn test_get_none() {
    let once_box: OnceBox<i32> = OnceBox::new();
    assert_eq!(once_box.get(), None);
}

#[test]
fn test_get_some() {
    let once_box = OnceBox::with_value(Box::new(42));
    let value = once_box.get();
    assert!(value.is_some());
    assert_eq!(*value.unwrap(), 42);
}

