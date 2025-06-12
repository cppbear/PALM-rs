// Answer 0

#[test]
fn test_force_mut_with_non_none_cell() {
    use once_cell::unsync::Lazy;
    use once_cell::OnceCell;

    struct Test;

    let func = || 42;
    let mut lazy = Lazy::new(func);
    let value_before = lazy.force_mut();

    // Ensure the value is properly initialized and we get the correct mutable reference
    assert_eq!(*value_before, 42); // before calling force_mut
    let value_after = lazy.force_mut();
    assert_eq!(*value_after, 42); // after calling force_mut again
}

#[test]
#[should_panic(expected = "Lazy instance has previously been poisoned")]
fn test_force_mut_should_panic_when_poisoned() {
    use once_cell::unsync::Lazy;
    use once_cell::OnceCell;

    struct Test;

    let mut lazy = Lazy::new(|| {
        panic!("Testing panic condition")
    });

    // This force_mut call will panic because the init function panics
    let _ = lazy.force_mut();
}

#[test]
fn test_force_mut_should_return_mutable_reference() {
    use once_cell::unsync::Lazy;
    use once_cell::OnceCell;

    struct Test;

    let mut lazy = Lazy::new(|| 100);
    let mutable_ref = lazy.force_mut();

    // Ensure we can modify the value through the mutable reference
    *mutable_ref += 1;
    assert_eq!(*lazy.force_mut(), 101);
}

