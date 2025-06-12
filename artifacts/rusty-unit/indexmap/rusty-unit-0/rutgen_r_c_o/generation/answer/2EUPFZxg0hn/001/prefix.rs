// Answer 0

#[test]
fn test_capacity_both_zero() {
    let index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.capacity();
}

#[test]
fn test_capacity_indices_zero_entries_non_zero() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.entries = Vec::with_capacity(10);
    index_map.capacity();
}

#[test]
fn test_capacity_indices_non_zero_entries_zero() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.indices = Indices::with_capacity(10);
    index_map.capacity();
}

#[test]
fn test_capacity_both_non_zero() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.indices = Indices::with_capacity(15);
    index_map.entries = Vec::with_capacity(20);
    index_map.capacity();
}

#[test]
fn test_capacity_indices_equal_entries() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.indices = Indices::with_capacity(25);
    index_map.entries = Vec::with_capacity(25);
    index_map.capacity();
}

#[test]
fn test_capacity_indices_max_capacity() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(1152921504606846975);
    index_map.capacity();
}

#[test]
fn test_capacity_entries_max_capacity() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(1152921504606846975);
    index_map.entries = Vec::with_capacity(1152921504606846975);
    index_map.capacity();
}

