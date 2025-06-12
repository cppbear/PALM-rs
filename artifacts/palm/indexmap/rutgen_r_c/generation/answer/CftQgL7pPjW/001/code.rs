// Answer 0

#[test]
fn test_swap_indices_within_bounds() {
    let mut index_set: IndexSet<u32, RandomState> = IndexSet {
        map: IndexMap {
            core: IndexMapCore::new(),
            hash_builder: RandomState::new(),
        },
    };
    
    index_set.map.insert(10);
    index_set.map.insert(20);
    index_set.map.insert(30);
    
    index_set.swap_indices(0, 1);
    
    assert_eq!(index_set.as_slice().to_vec(), vec![20, 10, 30]);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_swap_indices_out_of_bounds_low() {
    let mut index_set: IndexSet<u32, RandomState> = IndexSet {
        map: IndexMap {
            core: IndexMapCore::new(),
            hash_builder: RandomState::new(),
        },
    };

    index_set.map.insert(10);
    index_set.map.insert(20);

    index_set.swap_indices(usize::MAX, 1);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_swap_indices_out_of_bounds_high() {
    let mut index_set: IndexSet<u32, RandomState> = IndexSet {
        map: IndexMap {
            core: IndexMapCore::new(),
            hash_builder: RandomState::new(),
        },
    };

    index_set.map.insert(10);
    index_set.map.insert(20);

    index_set.swap_indices(0, 2);
}

#[test]
fn test_swap_indices_same_index() {
    let mut index_set: IndexSet<u32, RandomState> = IndexSet {
        map: IndexMap {
            core: IndexMapCore::new(),
            hash_builder: RandomState::new(),
        },
    };

    index_set.map.insert(10);
    index_set.map.insert(20);
    
    index_set.swap_indices(1, 1);
    
    assert_eq!(index_set.as_slice().to_vec(), vec![10, 20]);
}

