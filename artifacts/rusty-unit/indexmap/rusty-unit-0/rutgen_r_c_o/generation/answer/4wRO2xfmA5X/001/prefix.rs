// Answer 0

#[test]
fn test_swap_indices_valid_case() {
    let mut map: IndexMap<u32, u32, RandomState> = IndexMap::with_capacity(5);
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    map.insert(4, 40);
    map.insert(5, 50);
    map.swap_indices(1, 3);
}

#[test]
#[should_panic] 
fn test_swap_indices_out_of_bounds_a() {
    let mut map: IndexMap<u32, u32, RandomState> = IndexMap::with_capacity(5);
    map.insert(1, 10);
    map.insert(2, 20);
    map.swap_indices(5, 1);
}

#[test]
#[should_panic] 
fn test_swap_indices_out_of_bounds_b() {
    let mut map: IndexMap<u32, u32, RandomState> = IndexMap::with_capacity(5);
    map.insert(1, 10);
    map.insert(2, 20);
    map.swap_indices(1, 5);
}

#[test]
#[should_panic]
fn test_swap_indices_out_of_bounds_both() {
    let mut map: IndexMap<u32, u32, RandomState> = IndexMap::with_capacity(5);
    map.insert(1, 10);
    map.insert(2, 20);
    map.swap_indices(5, 6);
}

#[test]
fn test_swap_indices_non_adjacent() {
    let mut map: IndexMap<u32, u32, RandomState> = IndexMap::with_capacity(5);
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    map.insert(4, 40);
    map.insert(5, 50);
    map.swap_indices(0, 4);
}

#[test]
fn test_swap_indices_with_large_values() {
    let mut map: IndexMap<u32, u32, RandomState> = IndexMap::with_capacity(10);
    for i in 0..10 {
        map.insert(i, i * 10);
    }
    map.swap_indices(2, 7);
}

#[test]
fn test_swap_indices_same_index() {
    let mut map: IndexMap<u32, u32, RandomState> = IndexMap::with_capacity(5);
    map.insert(1, 10);
    map.insert(2, 20);
    map.swap_indices(1, 1);
}

