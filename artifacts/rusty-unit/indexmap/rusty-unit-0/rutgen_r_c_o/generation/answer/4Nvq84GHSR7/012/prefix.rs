// Answer 0

#[test]
fn test_erase_indices_case_does_nothing() {
    let mut index_map = IndexMapCore::<usize, usize>::new();
    index_map.clear(); // Ensure it is empty
    index_map.erase_indices(0, 0); // No entries to erase
}

#[test]
fn test_erase_indices_case_start_at_end() {
    let mut index_map = IndexMapCore::<usize, usize>::with_capacity(2);
    index_map.push_entry(HashValue(1), 1, 10);
    index_map.push_entry(HashValue(2), 2, 20);
    index_map.erase_indices(1, 1); // Erasing with start equal to erased 
}

#[test]
fn test_erase_indices_case_multiple_erased() {
    let mut index_map = IndexMapCore::<usize, usize>::with_capacity(3);
    index_map.push_entry(HashValue(1), 1, 10);
    index_map.push_entry(HashValue(2), 2, 20);
    index_map.push_entry(HashValue(3), 3, 30);
    index_map.erase_indices(1, 3); // start < erased, erased > 0
}

#[test]
fn test_erase_indices_case_half_capacity_adjustment() {
    let mut index_map = IndexMapCore::<usize, usize>::with_capacity(4);
    index_map.push_entry(HashValue(1), 1, 10);
    index_map.push_entry(HashValue(2), 2, 20);
    index_map.push_entry(HashValue(3), 3, 30);
    index_map.push_entry(HashValue(4), 4, 40);
    index_map.erase_indices(1, 3); // Adjusted indices for start + shifted < half_capacity
}

#[test]
fn test_erase_indices_case_full_sweep() {
    let mut index_map = IndexMapCore::<usize, usize>::with_capacity(4);
    index_map.push_entry(HashValue(1), 1, 10);
    index_map.push_entry(HashValue(2), 2, 20);
    index_map.push_entry(HashValue(3), 3, 30);
    index_map.push_entry(HashValue(4), 4, 40);
    index_map.erase_indices(0, 4); // Sweep entire table, modified length
}

