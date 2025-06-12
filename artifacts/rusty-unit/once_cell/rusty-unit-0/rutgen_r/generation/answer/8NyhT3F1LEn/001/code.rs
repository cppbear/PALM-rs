// Answer 0

#[test]
fn test_to_usize_true() {
    use std::num::NonZeroUsize;

    let result = to_usize(true);
    assert_eq!(result.get(), 1);
}

#[test]
fn test_to_usize_false() {
    use std::num::NonZeroUsize;
    
    let result = to_usize(false);
    assert_eq!(result.get(), 2);
}

