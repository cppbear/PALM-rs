// Answer 0

#[test]
fn test_swap_indices_valid() {
    struct TestEntry {
        index: usize,
        map: indexmap::IndexMap<usize, usize>,
    }

    let mut map = indexmap::IndexMap::new();
    map.insert(0, 10);
    map.insert(1, 20);
    map.insert(2, 30);

    let entry1 = TestEntry { index: 0, map: map.clone() };
    let entry2 = TestEntry { index: 1, map: map };

    entry1.map.swap_indices(entry1.index, entry2.index);

    assert_eq!(entry1.map.get(&0), Some(&20));
    assert_eq!(entry1.map.get(&1), Some(&10));
}

#[should_panic]
#[test]
fn test_swap_indices_invalid() {
    struct TestEntry {
        index: usize,
        map: indexmap::IndexMap<usize, usize>,
    }

    let mut map = indexmap::IndexMap::new();
    map.insert(0, 10);
    map.insert(1, 20);
    map.insert(2, 30);
    
    let entry1 = TestEntry { index: 0, map: map };

    // This should panic because index 3 is out of bounds
    entry1.map.swap_indices(entry1.index, 3);
}

