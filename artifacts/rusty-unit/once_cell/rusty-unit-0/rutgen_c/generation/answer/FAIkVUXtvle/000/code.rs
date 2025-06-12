// Answer 0

#[test]
fn test_once_bool_new() {
    let once_bool = OnceBool::new();
    assert_eq!(once_bool.inner.inner.load(Ordering::SeqCst), 0);
}

#[test]
fn test_once_non_zero_usize_new() {
    let once_non_zero_usize = OnceNonZeroUsize::new();
    assert_eq!(once_non_zero_usize.inner.load(Ordering::SeqCst), 0);
} 

#[should_panic]
fn test_once_bool_set_invalid() {
    let once_bool = OnceBool::new();
    once_bool.set(false).unwrap(); // Assuming first call should succeed
    once_bool.set(true).unwrap_err(); // This should panic or return an error
}

#[test]
fn test_once_non_zero_usize_set() {
    let once_non_zero_usize = OnceNonZeroUsize::new();
    let value = NonZeroUsize::new(1).unwrap();
    assert_eq!(once_non_zero_usize.set(value), Ok(()));
    assert_eq!(once_non_zero_usize.inner.load(Ordering::SeqCst), 1);
}

