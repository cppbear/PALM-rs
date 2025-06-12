// Answer 0

#[test]
fn test_to_usize_true() {
    use core::num::NonZeroUsize;

    // Call the function with value as true
    let result = OnceBool::to_usize(true);

    // Ensure the returned NonZeroUsize is 1
    assert_eq!(result.get(), Some(1));
}

#[test]
#[should_panic]
fn test_to_usize_false() {
    use core::num::NonZeroUsize;

    // Call the function with value as false
    // This should panic since it will return NonZeroUsize::new_unchecked(2) which is still a valid NonZeroUsize
    let _ = OnceBool::to_usize(false);
}

