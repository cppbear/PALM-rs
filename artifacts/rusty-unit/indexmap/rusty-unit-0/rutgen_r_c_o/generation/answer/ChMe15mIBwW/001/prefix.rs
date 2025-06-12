// Answer 0

#[test]
fn test_move_index_valid_bounds() {
    let mut index_map = IndexMapCore::with_capacity(10);
    index_map.entries.push(Bucket { hash: HashValue::from(1), key: 1, value: "A" });
    index_map.entries.push(Bucket { hash: HashValue::from(2), key: 2, value: "B" });
    index_map.move_index(0, 1);
}

#[test]
fn test_move_index_reverse_bounds() {
    let mut index_map = IndexMapCore::with_capacity(10);
    index_map.entries.push(Bucket { hash: HashValue::from(1), key: 1, value: "A" });
    index_map.entries.push(Bucket { hash: HashValue::from(2), key: 2, value: "B" });
    index_map.move_index(1, 0);
}

#[test]
#[should_panic]
fn test_move_index_out_of_bounds_from() {
    let mut index_map = IndexMapCore::new();
    index_map.move_index(0, 1);
}

#[test]
#[should_panic]
fn test_move_index_out_of_bounds_to() {
    let mut index_map = IndexMapCore::new();
    index_map.entries.push(Bucket { hash: HashValue::from(1), key: 1, value: "A" });
    index_map.move_index(0, 1);
}

#[test]
#[should_panic]
fn test_move_index_same_index() {
    let mut index_map = IndexMapCore::with_capacity(10);
    index_map.entries.push(Bucket { hash: HashValue::from(1), key: 1, value: "A" });
    index_map.move_index(0, 0);
}

#[test]
fn test_move_index_edge_case() {
    let mut index_map = IndexMapCore::with_capacity(10);
    for i in 0..10 {
        index_map.entries.push(Bucket { hash: HashValue::from(i), key: i, value: "A" });
    }
    index_map.move_index(0, 9);
}

