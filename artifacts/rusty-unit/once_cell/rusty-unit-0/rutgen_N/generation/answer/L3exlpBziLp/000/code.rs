// Answer 0

#[test]
fn test_get_mut_not_initialized() {
    use once_cell::unsync::Lazy;

    let mut lazy = Lazy::new(|| 42);
    
    assert_eq!(Lazy::get_mut(&mut lazy), None);
    assert_eq!(*lazy, 42);
    assert_eq!(Lazy::get_mut(&mut lazy), Some(&mut 42));
}

#[test]
fn test_get_mut_initialized() {
    use once_cell::unsync::Lazy;

    let mut lazy = Lazy::new(|| 13);
    let _ = *lazy; // Initialize the value

    assert_eq!(Lazy::get_mut(&mut lazy), Some(&mut 13));
}

#[test]
#[should_panic]
fn test_get_mut_after_moving_value() {
    use once_cell::unsync::Lazy;

    let mut lazy = Lazy::new(|| 27);
    let _ = *lazy; // Initialize the value

    let value = Lazy::get_mut(&mut lazy);
    let _ = value.unwrap(); // Get mutable reference

    // Attempting to access mutable again after it has been moved
    let _ = Lazy::get_mut(&mut lazy); // This should panic if accessed after move
}

