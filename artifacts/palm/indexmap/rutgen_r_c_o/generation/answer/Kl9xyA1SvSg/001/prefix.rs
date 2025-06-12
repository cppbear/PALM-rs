// Answer 0

#[test]
fn test_move_index_valid_upward_shift() {
    let mut index_set = IndexSet { map: IndexMap::new() };
    index_set.push(1);
    index_set.push(2);
    index_set.push(3);
    index_set.move_index(2, 1);
}

#[test]
fn test_move_index_valid_downward_shift() {
    let mut index_set = IndexSet { map: IndexMap::new() };
    index_set.push(1);
    index_set.push(2);
    index_set.push(3);
    index_set.move_index(0, 2);
}

#[test]
#[should_panic]
fn test_move_index_from_out_of_bounds() {
    let mut index_set = IndexSet { map: IndexMap::new() };
    index_set.push(1);
    index_set.push(2);
    index_set.push(3);
    index_set.move_index(3, 1);
}

#[test]
#[should_panic]
fn test_move_index_to_out_of_bounds() {
    let mut index_set = IndexSet { map: IndexMap::new() };
    index_set.push(1);
    index_set.push(2);
    index_set.push(3);
    index_set.move_index(1, 3);
}

#[test]
#[should_panic]
fn test_move_index_from_equals_to() {
    let mut index_set = IndexSet { map: IndexMap::new() };
    index_set.push(1);
    index_set.push(2);
    index_set.push(3);
    index_set.move_index(1, 1);
}

#[test]
fn test_move_index_on_two_elements() {
    let mut index_set = IndexSet { map: IndexMap::new() };
    index_set.push(1);
    index_set.push(2);
    index_set.move_index(0, 1);
}

