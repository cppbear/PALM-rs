// Answer 0

#[test]
fn test_get_mut_initial_not_initialized() {
    use once_cell::unsync::Lazy;

    let mut lazy: Lazy<i32, fn() -> i32> = Lazy::new(|| 92);
    
    assert_eq!(Lazy::get_mut(&mut lazy), None);
}

#[test]
fn test_get_mut_after_initialized() {
    use once_cell::unsync::Lazy;

    let mut lazy: Lazy<i32, fn() -> i32> = Lazy::new(|| 92);
    
    // Accessing the value initializes it
    assert_eq!(*lazy, 92);
    
    // Now get_mut should return Some reference
    assert_eq!(Lazy::get_mut(&mut lazy), Some(&mut 92));
}

#[test]
fn test_get_mut_after_double_initialization() {
    use once_cell::unsync::Lazy;

    let mut lazy: Lazy<i32, fn() -> i32> = Lazy::new(|| 42);
    
    // Initialize and access the value
    assert_eq!(*lazy, 42);
    
    // this check guarantees that get_mut is still valid after having accessed the value
    let mut_value = Lazy::get_mut(&mut lazy);
    assert!(mut_value.is_some());
    assert_eq!(*mut_value.unwrap(), 42);
}

