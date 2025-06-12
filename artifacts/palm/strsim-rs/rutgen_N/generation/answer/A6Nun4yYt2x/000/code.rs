// Answer 0

#[test]
fn test_flat_index() {
    assert_eq!(flat_index(0, 0, 4), 0);
    assert_eq!(flat_index(1, 0, 4), 1);
    assert_eq!(flat_index(3, 0, 4), 3);
    assert_eq!(flat_index(0, 1, 4), 4);
    assert_eq!(flat_index(1, 1, 4), 5);
    assert_eq!(flat_index(3, 1, 4), 7);
    assert_eq!(flat_index(0, 2, 4), 8);
    assert_eq!(flat_index(1, 2, 4), 9);
    assert_eq!(flat_index(3, 2, 4), 11);
    assert_eq!(flat_index(1, 3, 4), 13);
}

