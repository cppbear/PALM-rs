// Answer 0

#[test]
fn test_move_index_within_bounds() {
    struct TestEntries {
        entries: Vec<(usize, usize)>,
        hash: HashValue,
    }

    struct TestIndices {
        indices: Vec<usize>,
    }

    let mut entries = TestEntries { entries: vec![(0, 10), (1, 20), (2, 30)], hash: HashValue::new() };
    let mut indices = TestIndices { indices: vec![0, 1, 2] };

    let mut map = RefMut::new(&mut indices.indices, &mut entries.entries);
    let indexed_entry = IndexedEntry::new(&mut map.map, 1);
    
    indexed_entry.move_index(0);

    assert_eq!(indexed_entry.index(), 0);
    assert_eq!(entries.entries[0], (1, 20));
}

#[test]
#[should_panic]
fn test_move_index_out_of_bounds() {
    struct TestEntries {
        entries: Vec<(usize, usize)>,
        hash: HashValue,
    }

    struct TestIndices {
        indices: Vec<usize>,
    }

    let mut entries = TestEntries { entries: vec![(0, 10), (1, 20), (2, 30)], hash: HashValue::new() };
    let mut indices = TestIndices { indices: vec![0, 1, 2] };

    let mut map = RefMut::new(&mut indices.indices, &mut entries.entries);
    let indexed_entry = IndexedEntry::new(&mut map.map, 1);

    indexed_entry.move_index(3);  // Attempt to move to an index that is out of bounds
}

#[test]
fn test_move_index_edge_case() {
    struct TestEntries {
        entries: Vec<(usize, usize)>,
        hash: HashValue,
    }

    struct TestIndices {
        indices: Vec<usize>,
    }

    let mut entries = TestEntries { entries: vec![(0, 10), (1, 20), (2, 30)], hash: HashValue::new() };
    let mut indices = TestIndices { indices: vec![0, 1, 2] };

    let mut map = RefMut::new(&mut indices.indices, &mut entries.entries);
    let indexed_entry = IndexedEntry::new(&mut map.map, 0);
    
    indexed_entry.move_index(2);  // Move to the last index

    assert_eq!(indexed_entry.index(), 2);
    assert_eq!(entries.entries[2], (0, 10));
}

