// Answer 0

#[test]
fn test_pos_new_valid_index() {
    let index: usize = 100; // Valid index (less than MAX_SIZE)
    let hash = HashValue(42); // Arbitrary HashValue

    let pos = Pos::new(index, hash);
    assert_eq!(pos.index, index as Size);
    assert_eq!(pos.hash, hash);
}

#[test]
fn test_pos_new_boundary_index() {
    let index: usize = MAX_SIZE - 1; // Boundary index (MAX_SIZE - 1)
    let hash = HashValue(99); // Arbitrary HashValue

    let pos = Pos::new(index, hash);
    assert_eq!(pos.index, index as Size);
    assert_eq!(pos.hash, hash);
}

#[should_panic(expected = "assertion failed: index < MAX_SIZE")]
#[test]
fn test_pos_new_invalid_index_too_large() {
    let index: usize = MAX_SIZE; // Invalid index (equals MAX_SIZE)
    let hash = HashValue(55); // Arbitrary HashValue

    Pos::new(index, hash); // This should panic
}

