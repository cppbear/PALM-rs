// Answer 0

#[test]
fn test_move_index_valid() {
    let mut index_map: IndexMap<u32, u32, RandomState> = IndexMap::new();
    index_map.insert(0, 0);
    index_map.insert(1, 1);
    index_map.insert(2, 2);
    index_map.insert(3, 3);
    
    index_map.move_index(1, 2);
}

#[test]
fn test_move_index_shift_down() {
    let mut index_map: IndexMap<u32, u32, RandomState> = IndexMap::new();
    index_map.insert(0, 0);
    index_map.insert(1, 1);
    index_map.insert(2, 2);
    
    index_map.move_index(0, 2);
}

#[test]
fn test_move_index_shift_up() {
    let mut index_map: IndexMap<u32, u32, RandomState> = IndexMap::new();
    index_map.insert(0, 0);
    index_map.insert(1, 1);
    index_map.insert(2, 2);
    
    index_map.move_index(2, 0);
}

#[should_panic]
fn test_move_index_from_out_of_bounds() {
    let mut index_map: IndexMap<u32, u32, RandomState> = IndexMap::new();
    index_map.insert(0, 0);
    
    index_map.move_index(1, 0);
}

#[should_panic]
fn test_move_index_to_out_of_bounds() {
    let mut index_map: IndexMap<u32, u32, RandomState> = IndexMap::new();
    index_map.insert(0, 0);
    
    index_map.move_index(0, 1);
}

#[should_panic]
fn test_move_index_both_out_of_bounds() {
    let mut index_map: IndexMap<u32, u32, RandomState> = IndexMap::new();
    index_map.insert(0, 0);
    
    index_map.move_index(1, 2);
}

