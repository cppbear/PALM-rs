// Answer 0

#[test]
fn test_swap_remove_index_valid_index() {
    let mut index_set = super::IndexSet {
        map: super::IndexMap {
            core: super::IndexMapCore {
                // Assuming there's a way to initialize this with some values.
            },
            hash_builder: Default::default(),
        },
    };
    
    // Assuming there's already a method to initialize the IndexSet with some values
    index_set.map.insert(0, ());
    index_set.map.insert(1, ());
    
    let removed = index_set.swap_remove_index(0);
    assert_eq!(removed, Some(0));
    assert_eq!(index_set.len(), 1); // should be one element left
}

#[test] 
#[should_panic]
fn test_swap_remove_index_out_of_bounds_high() {
    let mut index_set = super::IndexSet {
        map: super::IndexMap {
            core: super::IndexMapCore {
                // Assuming there's a way to initialize this with some values.
            },
            hash_builder: Default::default(),
        },
    };

    // Assuming there's already a method to initialize the IndexSet with some values
    index_set.map.insert(0, ());

    // Attempting to remove an index that exceeds the length
    let _removed = index_set.swap_remove_index(1); // This should panic
}

#[test]
#[should_panic]
fn test_swap_remove_index_out_of_bounds_negative() {
    let mut index_set = super::IndexSet {
        map: super::IndexMap {
            core: super::IndexMapCore {
                // Assuming there's a way to initialize this with some values.
            },
            hash_builder: Default::default(),
        },
    };

    // Assuming there's already a method to initialize the IndexSet with some values
    index_set.map.insert(0, ());

    // Attempting to remove an invalid negative index
    let _removed = index_set.swap_remove_index(!0); // This should panic
}

#[test]
fn test_swap_remove_index_last_element() {
    let mut index_set = super::IndexSet {
        map: super::IndexMap {
            core: super::IndexMapCore {
                // Assuming there's a way to initialize this with some values.
            },
            hash_builder: Default::default(),
        },
    };

    index_set.map.insert(0, ());
    index_set.map.insert(1, ());

    let removed = index_set.swap_remove_index(1); // should remove last element
    assert_eq!(removed, Some(1));
    assert_eq!(index_set.len(), 1); // should be one element left
}

