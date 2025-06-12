// Answer 0

#[test]
fn test_erase_indices_empty_case() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.erase_indices(0, 0);
}

#[test]
fn test_erase_indices_reinsert_case() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(10);
    for i in 0..10 {
        index_map.entries.push(Bucket { hash: HashValue(i), key: i, value: i * 10 });
    }
    index_map.indices.push(0);
    index_map.indices.push(1);
    index_map.indices.push(2);
    index_map.indices.push(3);
    index_map.indices.push(4);
    index_map.indices.push(5);
    index_map.indices.push(6);
    index_map.indices.push(7);
    index_map.indices.push(8);
    index_map.indices.push(9);
    
    index_map.erase_indices(5, 10);
}

#[test]
fn test_erase_indices_adjust_indices_case() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(10);
    for i in 0..10 {
        index_map.entries.push(Bucket { hash: HashValue(i), key: i, value: i * 10 });
    }
    for i in 0..10 {
        index_map.indices.push(i);
    }

    index_map.erase_indices(3, 7);
}

#[test]
fn test_erase_indices_sweep_case() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(10);
    for i in 0..10 {
        index_map.entries.push(Bucket { hash: HashValue(i), key: i, value: i * 10 });
    }
    
    index_map.indices.push(0);
    index_map.indices.push(1);
    index_map.indices.push(2);
    index_map.indices.push(3);
    index_map.indices.push(4);
    index_map.indices.push(5);
    index_map.indices.push(6);
    index_map.indices.push(7);
    
    index_map.erase_indices(2, 8);
}

#[test]
fn test_erase_indices_edge_case() {
    let half_capacity = IndexMapCore::<usize, usize>::MAX_ENTRIES_CAPACITY / 2;
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(half_capacity * 2);
    for i in 0..half_capacity {
        index_map.entries.push(Bucket { hash: HashValue(i), key: i, value: i * 10 });
    }
    
    for i in 0..half_capacity {
        index_map.indices.push(i);
    }

    index_map.erase_indices(half_capacity / 2, half_capacity);
}

