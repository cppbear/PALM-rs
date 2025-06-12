// Answer 0

#[test]
fn test_new_valid_input() {
    let index: usize = 5;
    let hash = HashValue::new(42); // Assuming HashValue has a method new
    let pos = new(index, hash);
    assert_eq!(pos.index, index as Size);
}

#[test]
#[should_panic]
fn test_new_index_out_of_bounds() {
    let index: usize = MAX_SIZE; // Assuming MAX_SIZE is defined
    let hash = HashValue::new(42);
    new(index, hash); // This should panic
}

