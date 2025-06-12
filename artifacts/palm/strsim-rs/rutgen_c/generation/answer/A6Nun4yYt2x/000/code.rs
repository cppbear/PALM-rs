// Answer 0

#[test]
fn test_flat_index() {
    assert_eq!(flat_index(0, 0, 1), 0);
    assert_eq!(flat_index(1, 0, 1), 1);
    assert_eq!(flat_index(0, 1, 1), 1);
    assert_eq!(flat_index(1, 1, 1), 2);
    assert_eq!(flat_index(0, 0, 3), 0);
    assert_eq!(flat_index(1, 0, 3), 1);
    assert_eq!(flat_index(2, 0, 3), 2);
    assert_eq!(flat_index(0, 1, 3), 3);
    assert_eq!(flat_index(1, 1, 3), 4);
    assert_eq!(flat_index(2, 1, 3), 5);
    assert_eq!(flat_index(0, 2, 3), 6);
    assert_eq!(flat_index(1, 2, 3), 7);
    assert_eq!(flat_index(2, 2, 3), 8);
}

#[test]
fn test_flat_index_zero_width() {
    assert_eq!(flat_index(0, 0, 0), 0);
}

#[test]
fn test_flat_index_large_values() {
    assert_eq!(flat_index(usize::MAX, usize::MAX, 1), usize::MAX);
    assert_eq!(flat_index(1, usize::MAX, 1), usize::MAX + 1);
}

