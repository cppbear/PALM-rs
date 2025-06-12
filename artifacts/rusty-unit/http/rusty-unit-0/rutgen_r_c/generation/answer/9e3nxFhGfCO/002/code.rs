// Answer 0

#[test]
#[should_panic]
fn test_new_index_equal_to_max_size() {
    let index = MAX_SIZE; // This should trigger the panic
    let hash = HashValue(1);
    Pos::new(index, hash);
}

#[test]
fn test_new_index_less_than_max_size() {
    let index = MAX_SIZE - 1; // This is within the valid range
    let hash = HashValue(1);
    let pos = Pos::new(index, hash);
    assert_eq!(pos.index, index as Size);
    assert_eq!(pos.hash, hash);
}

