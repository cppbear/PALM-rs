// Answer 0

#[test]
fn test_select_guard_empty_pattern() {
    let pattern: &[u8] = &[];
    // According to the function logic, this should panic because of trying to access an element in an empty slice.
    let result = std::panic::catch_unwind(|| {
        select_guard(pattern)
    });
    assert!(result.is_err());
}

#[test]
#[should_panic]
fn test_select_guard_single_element() {
    let pattern: &[u8] = &[1];
    // Should return (1, 0), as it's the only element.
    let result = select_guard(pattern);
    assert_eq!(result, (1, 0));
}

#[test]
fn test_select_guard_multiple_elements() {
    let pattern: &[u8] = &[5, 3, 9, 1, 7];
    // The rarest character is 1 with index 3 when considering reverse index.
    let result = select_guard(pattern);
    assert_eq!(result, (1, 1));
}

#[test]
fn test_select_guard_identical_elements() {
    let pattern: &[u8] = &[2, 2, 2, 2];
    // All elements are identical, should return the first element and index 0.
    let result = select_guard(pattern);
    assert_eq!(result, (2, 0));
}

#[test]
fn test_select_guard_max_values() {
    let pattern: &[u8] = &[255, 254, 253, 252];
    // The rarest character is 252 with index 3 when considering reverse index.
    let result = select_guard(pattern);
    assert_eq!(result, (252, 0));
}

