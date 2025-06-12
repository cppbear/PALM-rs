// Answer 0

#[test]
fn test_once_ref_new() {
    let once_ref: OnceRef<i32> = OnceRef::new();
    assert_eq!(once_ref.inner.load(Ordering::SeqCst), std::ptr::null_mut());
}

#[test]
fn test_once_ref_new_with_different_type() {
    let once_ref: OnceRef<f64> = OnceRef::new();
    assert_eq!(once_ref.inner.load(Ordering::SeqCst), std::ptr::null_mut());
}

#[test]
fn test_once_ref_new_with_lifetime() {
    let once_ref: OnceRef<'static, String> = OnceRef::new();
    assert_eq!(once_ref.inner.load(Ordering::SeqCst), std::ptr::null_mut());
}

