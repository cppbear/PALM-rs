// Answer 0

#[test]
fn test_compare_exchange_success() {
    let once = OnceNonZeroUsize::new();
    let non_zero_value = NonZeroUsize::new(10).unwrap();
    
    let result = once.compare_exchange(non_zero_value);
    assert_eq!(result, Ok(0));
}

#[test]
fn test_compare_exchange_failure() {
    let once = OnceNonZeroUsize::new();
    let non_zero_value_1 = NonZeroUsize::new(10).unwrap();
    let non_zero_value_2 = NonZeroUsize::new(20).unwrap();
    
    // First successful exchange
    let _ = once.compare_exchange(non_zero_value_1);
    
    // Now trying to compare and exchange with a different value should fail
    let result = once.compare_exchange(non_zero_value_2);
    assert_eq!(result, Err(10));
}

#[test]
#[should_panic]
fn test_compare_exchange_zero_value() {
    let once = OnceNonZeroUsize::new();
    let zero_value = NonZeroUsize::new(0).unwrap(); // This should not happen since NonZeroUsize cannot be zero
    let _ = once.compare_exchange(zero_value);
}

