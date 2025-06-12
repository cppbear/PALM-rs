// Answer 0

#[test]
fn test_move_index_up() {
    let mut map = indexmap::IndexMap::new();
    map.insert(0, "A");
    map.insert(1, "B");
    map.insert(2, "C");
    map.move_index(2, 0);
    let expected: Vec<_> = map.keys().cloned().collect();
    assert_eq!(expected, vec!["C", "A", "B"]);
}

#[test]
fn test_move_index_down() {
    let mut map = indexmap::IndexMap::new();
    map.insert(0, "A");
    map.insert(1, "B");
    map.insert(2, "C");
    map.move_index(0, 2);
    let expected: Vec<_> = map.keys().cloned().collect();
    assert_eq!(expected, vec!["B", "C", "A"]);
}

#[test]
#[should_panic]
fn test_move_index_out_of_bounds_low() {
    let mut map = indexmap::IndexMap::new();
    map.insert(0, "A");
    map.move_index(0, 1);
}

#[test]
#[should_panic]
fn test_move_index_out_of_bounds_high() {
    let mut map = indexmap::IndexMap::new();
    map.insert(0, "A");
    map.move_index(1, 0);
}

