// Answer 0

#[test]
fn test_once_box_with_value() {
    struct TestData {
        value: i32,
    }

    let boxed_value = Box::new(TestData { value: 42 });
    let once_box = OnceBox::with_value(boxed_value);

    unsafe {
        assert!(!once_box.inner.load(Ordering::SeqCst).is_null());
        let ptr = once_box.inner.load(Ordering::SeqCst);
        let data = &*ptr;
        assert_eq!(data.value, 42);
    }
}

#[test]
fn test_once_box_new() {
    let once_box: OnceBox<i32> = OnceBox::new();

    unsafe {
        assert!(once_box.inner.load(Ordering::SeqCst).is_null());
    }
}

