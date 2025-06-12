// Answer 0

#[test]
fn test_get_unchecked_initialization() {
    use core::num::NonZeroUsize;

    // Initialize a OnceNonZeroUsize
    let once_non_zero = OnceNonZeroUsize::new();

    // Set a valid NonZeroUsize value
    let non_zero_value = NonZeroUsize::new(42).unwrap();
    let _ = once_non_zero.set(non_zero_value);

    // Call unsafe `get_unchecked()`
    unsafe {
        let result = once_non_zero.get_unchecked();
        assert_eq!(result.get(), 42);
    }
}

#[test]
#[should_panic]
fn test_get_unchecked_uninitialized() {
    use core::num::NonZeroUsize;

    // Create a new OnceNonZeroUsize without initializing
    let once_non_zero = OnceNonZeroUsize::new();

    // Call unsafe `get_unchecked()`, should panic because it's uninitialized
    unsafe {
        let _ = once_non_zero.get_unchecked();
    }
}

