// Answer 0

#[test]
fn test_get_index_mut_valid_index() {
    let mut index_map = IndexMap::new();
    index_map.insert("a", 1);
    index_map.insert("b", 2);
    index_map.insert("c", 3);
    let _ = index_map.get_index_mut(0);
    let _ = index_map.get_index_mut(1);
    let _ = index_map.get_index_mut(2);
}

#[test]
fn test_get_index_mut_edge_case_first() {
    let mut index_map = IndexMap::new();
    index_map.insert("x", 10);
    let _ = index_map.get_index_mut(0);
}

#[test]
fn test_get_index_mut_edge_case_last() {
    let mut index_map = IndexMap::new();
    index_map.insert("y", 20);
    let _ = index_map.get_index_mut(0);
}

#[should_panic]
fn test_get_index_mut_out_of_bounds_high() {
    let mut index_map = IndexMap::new();
    index_map.insert("m", 5);
    let _ = index_map.get_index_mut(1);
}

#[should_panic]
fn test_get_index_mut_out_of_bounds_zero() {
    let mut index_map = IndexMap::new();
    let _ = index_map.get_index_mut(0);
}

#[test]
fn test_get_index_mut_large_map() {
    let mut index_map: IndexMap<usize, usize, RandomState> = IndexMap::with_capacity(1000);
    for i in 0..1000 {
        index_map.insert(i, i * 10);
    }
    for i in 0..1000 {
        let _ = index_map.get_index_mut(i);
    }
}

#[should_panic]
fn test_get_index_mut_empty_map() {
    let mut index_map = IndexMap::new();
    let _ = index_map.get_index_mut(0);
}

