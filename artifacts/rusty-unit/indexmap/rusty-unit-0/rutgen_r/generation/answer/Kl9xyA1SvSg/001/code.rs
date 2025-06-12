// Answer 0

#[test]
fn test_move_index_within_bounds() {
    let mut index_map = indexmap::IndexMap::new();
    index_map.insert("a", 1);
    index_map.insert("b", 2);
    index_map.insert("c", 3);

    index_map.move_index(0, 1);
    assert_eq!(index_map.keys().collect::<Vec<_>>(), vec!["b", "a", "c"]);

    index_map.move_index(1, 2);
    assert_eq!(index_map.keys().collect::<Vec<_>>(), vec!["b", "c", "a"]);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_move_index_from_out_of_bounds() {
    let mut index_map = indexmap::IndexMap::new();
    index_map.insert("a", 1);
    index_map.move_index(1, 0); // from is out of bounds
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_move_index_to_out_of_bounds() {
    let mut index_map = indexmap::IndexMap::new();
    index_map.insert("a", 1);
    index_map.move_index(0, 1); // to is out of bounds
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_move_index_both_out_of_bounds() {
    let mut index_map = indexmap::IndexMap::new();
    index_map.insert("a", 1);
    index_map.move_index(1, 2); // both are out of bounds
}

#[test]
fn test_move_index_edge_case_same_index() {
    let mut index_map = indexmap::IndexMap::new();
    index_map.insert("a", 1);
    index_map.move_index(0, 0); // moving the element to the same index
    assert_eq!(index_map.keys().collect::<Vec<_>>(), vec!["a"]);
}

