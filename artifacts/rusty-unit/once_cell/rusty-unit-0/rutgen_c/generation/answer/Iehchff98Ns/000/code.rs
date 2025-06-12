// Answer 0

#[test]
fn test_get_or_init_empty_cell() {
    use core::num::NonZeroUsize;

    let cell = OnceNonZeroUsize::new();
    
    let result = cell.get_or_init(|| NonZeroUsize::new(42).unwrap());
    
    assert_eq!(result.get(), 42);
}

#[test]
fn test_get_or_init_non_empty_cell() {
    use core::num::NonZeroUsize;

    let cell = OnceNonZeroUsize::new();
    
    // Initialize the cell first
    let _ = cell.set(NonZeroUsize::new(42).unwrap()).expect("Should succeed");

    let result = cell.get_or_init(|| NonZeroUsize::new(100).unwrap());

    assert_eq!(result.get(), 42);
}

#[test]
#[should_panic]
fn test_get_or_init_with_zero() {
    use core::num::NonZeroUsize;

    let cell = OnceNonZeroUsize::new();
    
    // This will panic because NonZeroUsize cannot be zero
    let _ = cell.get_or_init(|| NonZeroUsize::new(0).unwrap());
}

