// Answer 0

#[test]
fn test_once_bool_new() {
    let once_bool = OnceBool::new();
    assert_eq!(once_bool.inner.inner.load(Ordering::SeqCst), 0);
}

