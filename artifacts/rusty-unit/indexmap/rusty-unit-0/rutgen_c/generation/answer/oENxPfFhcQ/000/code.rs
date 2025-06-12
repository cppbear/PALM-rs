// Answer 0

#[test]
fn test_splice_new_valid_range() {
    use std::collections::hash_map::RandomState;

    // Initialize a mutable IndexMap
    let mut map: IndexMap<u32, String, RandomState> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: RandomState::new(),
    };

    // Populate the map
    map.insert(0, "zero".to_string());
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());

    // Create an iterator for replacement
    let replace_with = vec![(3, "three".to_string()), (4, "four".to_string())].into_iter();

    // Create a splice using valid range
    let splice = Splice::new(&mut map, 0..2, replace_with);

    // Check the contents of the splice
    assert_eq!(splice.tail.entries.len(), 2); // Expecting 2 elements in the tail
    assert_eq!(splice.drain.len(), 2); // Expecting 2 elements in the drain
}

#[test]
fn test_splice_new_empty_range() {
    use std::collections::hash_map::RandomState;

    // Initialize a mutable IndexMap
    let mut map: IndexMap<u32, String, RandomState> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: RandomState::new(),
    };

    // Populate the map
    map.insert(0, "zero".to_string());
    map.insert(1, "one".to_string());

    // Create an iterator for replacement
    let replace_with = vec![(2, "two".to_string())].into_iter();

    // Create a splice using an empty range
    let splice = Splice::new(&mut map, 2..2, replace_with);

    // Check the contents of the splice
    assert_eq!(splice.tail.entries.len(), 0); // Expecting an empty tail
    assert_eq!(splice.drain.len(), 0); // Expecting an empty drain
}

#[test]
#[should_panic]
fn test_splice_new_out_of_bounds_range() {
    use std::collections::hash_map::RandomState;

    // Initialize a mutable IndexMap
    let mut map: IndexMap<u32, String, RandomState> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: RandomState::new(),
    };

    // Populate the map
    map.insert(0, "zero".to_string());
    map.insert(1, "one".to_string());

    // Create an iterator for replacement
    let replace_with = vec![(2, "two".to_string())].into_iter();

    // Attempt to create a splice using out-of-bounds range, expecting a panic
    let _splice = Splice::new(&mut map, 0..3, replace_with);
}

