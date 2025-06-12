// Answer 0

#[test]
fn test_get_mut_uninitialized() {
    use once_cell::unsync::Lazy;

    // Create an uninitialized Lazy instance
    let mut lazy: Lazy<i32, fn() -> i32> = Lazy::new(|| 92);

    // Initial state should return None from get_mut
    assert_eq!(Lazy::get_mut(&mut lazy), None);
}

#[test]
fn test_get_mut_initialized() {
    use once_cell::unsync::Lazy;

    // Create a Lazy instance, which will immediately initialize
    let mut lazy = Lazy::new(|| 92);

    // Initialize the Lazy instance
    let _ = *lazy;

    // After initialization, get_mut should return a mutable reference to the value
    let mut_value = Lazy::get_mut(&mut lazy).expect("Expected to get a mutable reference");
    *mut_value += 1; // Modify the value

    // Ensure the value is updated
    assert_eq!(*lazy, 93);
}

#[test]
fn test_get_mut_multiple_calls() {
    use once_cell::unsync::Lazy;

    let mut lazy = Lazy::new(|| 42);
    let _ = *lazy;

    // Call get_mut multiple times
    let mut_ref1 = Lazy::get_mut(&mut lazy).expect("Expected to get a mutable reference");
    *mut_ref1 += 1;

    let mut_ref2 = Lazy::get_mut(&mut lazy).expect("Expected to get a mutable reference");
    *mut_ref2 += 1;

    assert_eq!(*lazy, 44);
}

#[test]
#[should_panic]
fn test_get_mut_after_reborrow() {
    use once_cell::unsync::Lazy;

    let mut lazy = Lazy::new(|| 10);
    let _ = *lazy;

    {
        let _mut_ref = Lazy::get_mut(&mut lazy).expect("Expected to get a mutable reference");
        *(_mut_ref) += 5;
    }

    // Attempting to use get_mut after creating a mutable reference should panic
    let _ = Lazy::get_mut(&mut lazy);
}

