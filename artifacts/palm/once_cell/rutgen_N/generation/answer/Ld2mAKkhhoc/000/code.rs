// Answer 0

#[test]
fn test_set_empty_cell() {
    use once_cell::unsync::OnceCell;

    let cell = OnceCell::new();
    assert!(cell.get().is_none());
    assert_eq!(cell.set(92), Ok(()));
    assert!(cell.get().is_some());
}

#[test]
fn test_set_full_cell() {
    use once_cell::unsync::OnceCell;

    let cell = OnceCell::new();
    assert_eq!(cell.set(92), Ok(()));
    assert_eq!(cell.set(62), Err(62));
    assert!(cell.get().is_some());
}

#[test]
fn test_set_multiple() {
    use once_cell::unsync::OnceCell;

    let cell = OnceCell::new();
    assert_eq!(cell.set(1), Ok(()));
    assert_eq!(cell.set(2), Err(2));
    assert_eq!(cell.set(3), Err(3));
}

#[test]
fn test_set_boundary_condition() {
    use once_cell::unsync::OnceCell;

    let cell = OnceCell::new();
    assert_eq!(cell.set(0), Ok(())); // Testing with a zero value
    assert_eq!(cell.set(0), Err(0)); // Testing setting the same zero value again
    assert!(cell.get().is_some());
}

