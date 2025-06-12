// Answer 0

#[test]
fn test_force_with_initialized_lazy() {
    use once_cell::sync::Lazy;

    // Create an initialized Lazy instance
    let lazy = Lazy::new(|| 42);
    
    // Ensure force returns a reference to the initialized value
    assert_eq!(*force(&lazy), 42);
    assert_eq!(&*lazy, &42);
}

#[test]
#[should_panic(expected = "Lazy instance has previously been poisoned")]
fn test_force_with_poisoned_lazy() {
    use once_cell::sync::Lazy;

    // Create a poisoned Lazy instance by initializing and consuming it
    let mut lazy = Lazy::new(|| 100);
    let _value = force(&lazy);

    // After calling force once, it should panic if we call it again
    // as it takes the initializer and thus would attempt to use a None value.
    force(&lazy);
}

