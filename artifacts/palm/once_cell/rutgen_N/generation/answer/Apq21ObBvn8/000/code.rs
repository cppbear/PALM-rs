// Answer 0

#[test]
fn test_get_mut_initialized() {
    use once_cell::sync::Lazy;

    let mut lazy = Lazy::new(|| 92);

    assert_eq!(Lazy::get_mut(&mut lazy), None);
    assert_eq!(&*lazy, &92);
    assert_eq!(Lazy::get_mut(&mut lazy), Some(&mut 92));
}

#[test]
fn test_get_mut_uninitialized() {
    use once_cell::sync::Lazy;

    let mut lazy = Lazy::new(|| 42);
    drop(lazy); // Drop the lazy value to simulate non-initialization

    assert_eq!(Lazy::get_mut(&mut lazy), None);
} 

#[test]
#[should_panic]
fn test_get_mut_on_reinitialized() {
    use once_cell::sync::Lazy;

    let mut lazy = Lazy::new(|| 50);
    let _ = Lazy::get_mut(&mut lazy); // Get mutable reference first

    *lazy = Lazy::new(|| 100); // Trying to reinitialize
}

