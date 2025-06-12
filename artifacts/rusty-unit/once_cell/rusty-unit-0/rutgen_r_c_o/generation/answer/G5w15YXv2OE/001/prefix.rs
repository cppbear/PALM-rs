// Answer 0

#[test]
fn test_set_err_full() {
    use core::num::NonZeroUsize;

    let once_non_zero = OnceNonZeroUsize::new();
    let initial_value = NonZeroUsize::new(1).unwrap();
    
    // Set an initial value to make the cell full
    let _ = once_non_zero.set(initial_value);
    
    // Attempt to set another value should result in Err
    let new_value = NonZeroUsize::new(2).unwrap();
    let result = once_non_zero.set(new_value);
}

#[test]
fn test_set_err_full_large_value() {
    use core::num::NonZeroUsize;

    let once_non_zero = OnceNonZeroUsize::new();
    let initial_value = NonZeroUsize::new(usize::MAX).unwrap();
    
    // Set an initial value to make the cell full
    let _ = once_non_zero.set(initial_value);
    
    // Attempt to set another value should result in Err
    let new_value = NonZeroUsize::new(1).unwrap();
    let result = once_non_zero.set(new_value);
}

#[test]
fn test_set_err_full_multiple_attempts() {
    use core::num::NonZeroUsize;

    let once_non_zero = OnceNonZeroUsize::new();
    let initial_value = NonZeroUsize::new(1).unwrap();
    
    // Set an initial value to make the cell full
    let _ = once_non_zero.set(initial_value);
    
    // Attempt to set different values should all result in Err
    for i in 2..5 {
        let new_value = NonZeroUsize::new(i).unwrap();
        let result = once_non_zero.set(new_value);
    }
}

