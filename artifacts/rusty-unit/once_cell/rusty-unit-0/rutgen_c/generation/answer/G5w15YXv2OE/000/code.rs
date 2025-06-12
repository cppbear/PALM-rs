// Answer 0

#[test]
fn test_set_success() {
    use core::num::NonZeroUsize;

    let once = OnceNonZeroUsize::new();
    let value = NonZeroUsize::new(42).unwrap();
    
    let result = once.set(value);
    
    assert_eq!(result, Ok(()));
}

#[test]
fn test_set_failure() {
    use core::num::NonZeroUsize;

    let once = OnceNonZeroUsize::new();
    let value1 = NonZeroUsize::new(42).unwrap();
    let value2 = NonZeroUsize::new(100).unwrap();
    
    let result1 = once.set(value1);
    let result2 = once.set(value2);
    
    assert_eq!(result1, Ok(()));
    assert_eq!(result2, Err(()));
}

