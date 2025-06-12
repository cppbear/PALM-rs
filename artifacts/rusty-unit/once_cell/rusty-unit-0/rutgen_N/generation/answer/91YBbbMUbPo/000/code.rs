// Answer 0

#[test]
fn test_force_mut_initialization() {
    use once_cell::unsync::Lazy;
    use once_cell::unsync::OnceCell;

    let mut lazy = Lazy::new(|| 42);

    let value_ref = force_mut(&mut lazy);
    assert_eq!(*value_ref, 42);
    assert_eq!(*lazy, 42);
}

#[test]
fn test_force_mut_with_poisoned_instance() {
    use once_cell::unsync::Lazy;
    use once_cell::unsync::OnceCell;

    let mut lazy = Lazy::new(|| panic!("poisoned"));

    let result = std::panic::catch_unwind(|| {
        force_mut(&mut lazy);
    });
    
    assert!(result.is_err());
}

