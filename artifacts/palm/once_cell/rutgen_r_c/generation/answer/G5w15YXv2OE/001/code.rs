// Answer 0

#[test]
fn test_set_when_full() {
    use core::num::NonZeroUsize;

    // Initialize OnceNonZeroUsize
    let once = OnceNonZeroUsize::new();
    let value = NonZeroUsize::new(1).unwrap();

    // First, we have to set a value to make the cell "full"
    let _ = once.set(value); // Initial set, should be Ok(())

    // Now we attempt to set a different value while it is "full"
    let new_value = NonZeroUsize::new(2).unwrap();
    let result = once.set(new_value);

    // The result should be an Err(()) since the cell is already full
    assert_eq!(result, Err(()));
}

