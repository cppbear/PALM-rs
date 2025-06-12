// Answer 0

#[test]
fn test_compare_exchange_success() {
    use core::num::NonZeroUsize;

    let once = OnceNonZeroUsize::new();
    let non_zero_val = NonZeroUsize::new(5).unwrap();
    
    let result = once.compare_exchange(non_zero_val);
    assert_eq!(result, Ok(0));
}

#[test]
fn test_compare_exchange_failure() {
    use core::num::NonZeroUsize;

    let once = OnceNonZeroUsize::new();
    let non_zero_val_1 = NonZeroUsize::new(5).unwrap();
    let non_zero_val_2 = NonZeroUsize::new(10).unwrap();

    let _ = once.compare_exchange(non_zero_val_1);
    let result = once.compare_exchange(non_zero_val_2);
    assert_eq!(result, Err(0));
}

#[test]
fn test_compare_exchange_multiple_success() {
    use core::num::NonZeroUsize;

    let once = OnceNonZeroUsize::new();
    let non_zero_val_1 = NonZeroUsize::new(3).unwrap();
    let non_zero_val_2 = NonZeroUsize::new(4).unwrap();

    let result_1 = once.compare_exchange(non_zero_val_1);
    assert_eq!(result_1, Ok(0));
    
    let result_2 = once.compare_exchange(non_zero_val_2);
    assert_eq!(result_2, Err(3));
}

