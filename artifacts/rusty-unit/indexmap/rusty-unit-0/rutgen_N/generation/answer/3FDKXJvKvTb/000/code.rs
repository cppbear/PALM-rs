// Answer 0

#[test]
fn test_swap_indices_within_bounds() {
    struct TestEntry {
        index: usize,
        map: indexmap::IndexMap<usize, usize>,
    }

    let mut map = indexmap::IndexMap::new();
    map.insert(0, 10);
    map.insert(1, 20);
    map.insert(2, 30);
    
    let mut entry = TestEntry { index: 0, map };

    entry.swap_indices(1);
    
    assert_eq!(entry.map.get_index(0).unwrap().0, 20);
    assert_eq!(entry.map.get_index(1).unwrap().0, 10);
}

#[test]
#[should_panic]
fn test_swap_indices_out_of_bounds() {
    struct TestEntry {
        index: usize,
        map: indexmap::IndexMap<usize, usize>,
    }

    let mut map = indexmap::IndexMap::new();
    map.insert(0, 10);
    map.insert(1, 20);
    
    let entry = TestEntry { index: 0, map };

    entry.swap_indices(2);
}

