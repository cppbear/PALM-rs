// Answer 0

#[test]
fn test_once_box_new() {
    let once_box: OnceBox<i32> = OnceBox::new();
    assert_eq!(unsafe { once_box.inner.load(Ordering::SeqCst) }, std::ptr::null_mut());
}

#[test]
fn test_once_box_new_with_different_type() {
    let once_box: OnceBox<String> = OnceBox::new();
    assert_eq!(unsafe { once_box.inner.load(Ordering::SeqCst) }, std::ptr::null_mut());
}

