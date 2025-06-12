// Answer 0

#[test]
#[should_panic]
fn test_new_with_max_size_index() {
    let index = 16384; // MAX_SIZE
    let hash = HashValue(0);
    Pos::new(index, hash);
}

