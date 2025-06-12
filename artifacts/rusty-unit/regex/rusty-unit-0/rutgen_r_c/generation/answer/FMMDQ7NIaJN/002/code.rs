// Answer 0

#[test]
fn test_select_guard_with_single_element() {
    let pattern: Vec<u8> = vec![b'a'];
    let (guard, reverse_idx) = select_guard(&pattern);
    assert_eq!(guard, b'a');
    assert_eq!(reverse_idx, 0);
}

#[test]
fn test_select_guard_with_identical_elements() {
    let pattern: Vec<u8> = vec![b'a', b'a', b'a'];
    let (guard, reverse_idx) = select_guard(&pattern);
    assert_eq!(guard, b'a');
    assert_eq!(reverse_idx, 2);
}

#[test]
fn test_select_guard_with_distinct_frequencies() {
    let pattern: Vec<u8> = vec![b'b', b'a', b'c'];
    let (guard, reverse_idx) = select_guard(&pattern);
    assert_eq!(guard, b'a');
    assert_eq!(reverse_idx, 1);
}

#[test]
#[should_panic]
fn test_select_guard_with_empty_pattern() {
    let pattern: Vec<u8> = vec![];
    select_guard(&pattern);
}

#[test]
fn test_select_guard_with_equal_frequencies() {
    let pattern: Vec<u8> = vec![b'a', b'b', b'a'];
    let (guard, reverse_idx) = select_guard(&pattern);
    assert_eq!(guard, b'b');
    assert_eq!(reverse_idx, 1);
}

