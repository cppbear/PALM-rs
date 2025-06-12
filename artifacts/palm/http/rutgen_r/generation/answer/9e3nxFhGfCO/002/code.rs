// Answer 0

#[derive(Debug)]
struct HashValue; // Assuming HashValue is a struct

const MAX_SIZE: usize = 10; // Assuming MAX_SIZE is defined as a constant

#[derive(Debug)]
struct Pos {
    index: usize,
    hash: HashValue,
}

impl Pos {
    fn new(index: usize, hash: HashValue) -> Self {
        debug_assert!(index < MAX_SIZE);
        Pos {
            index,
            hash,
        }
    }
}

#[test]
#[should_panic(expected = "assertion failed: index < MAX_SIZE")]
fn test_new_with_index_equal_to_max_size() {
    let hash_value = HashValue;
    let _pos = Pos::new(MAX_SIZE, hash_value);
}

#[test]
#[should_panic(expected = "assertion failed: index < MAX_SIZE")]
fn test_new_with_index_greater_than_max_size() {
    let hash_value = HashValue;
    let _pos = Pos::new(MAX_SIZE + 1, hash_value);
}

#[test]
fn test_new_with_index_less_than_max_size() {
    let hash_value = HashValue;
    let pos = Pos::new(MAX_SIZE - 1, hash_value);
    assert_eq!(pos.index, MAX_SIZE - 1);
}

