// Answer 0

#[test]
fn test_swap_remove_index_valid_index() {
    let mut index_set: IndexSet<u32, RandomState> = IndexSet {
        map: IndexMap::new(),
    };
    index_set.map.insert(0, ());
    index_set.map.insert(1, ());
    index_set.map.insert(2, ());
    let _ = index_set.swap_remove_index(1);
}

#[test]
fn test_swap_remove_index_empty() {
    let mut index_set: IndexSet<u32, RandomState> = IndexSet {
        map: IndexMap::new(),
    };
    let _ = index_set.swap_remove_index(0);
}

#[test]
fn test_swap_remove_index_out_of_bounds_high() {
    let mut index_set: IndexSet<u32, RandomState> = IndexSet {
        map: IndexMap::new(),
    };
    index_set.map.insert(0, ());
    index_set.map.insert(1, ());
    let _ = index_set.swap_remove_index(10);
}

#[test]
fn test_swap_remove_index_out_of_bounds_low() {
    let mut index_set: IndexSet<u32, RandomState> = IndexSet {
        map: IndexMap::new(),
    };
    index_set.map.insert(0, ());
    let _ = index_set.swap_remove_index(usize::MAX);
}

#[test]
fn test_swap_remove_index_multiple_elements() {
    let mut index_set: IndexSet<u32, RandomState> = IndexSet {
        map: IndexMap::new(),
    };
    index_set.map.insert(0, ());
    index_set.map.insert(1, ());
    index_set.map.insert(2, ());
    index_set.map.insert(3, ());
    let _ = index_set.swap_remove_index(2);
}

#[test]
fn test_swap_remove_index_first_element() {
    let mut index_set: IndexSet<u32, RandomState> = IndexSet {
        map: IndexMap::new(),
    };
    index_set.map.insert(0, ());
    index_set.map.insert(1, ());
    let _ = index_set.swap_remove_index(0);
}

#[test]
fn test_swap_remove_index_last_element() {
    let mut index_set: IndexSet<u32, RandomState> = IndexSet {
        map: IndexMap::new(),
    };
    index_set.map.insert(0, ());
    index_set.map.insert(1, ());
    let _ = index_set.swap_remove_index(1);
}

