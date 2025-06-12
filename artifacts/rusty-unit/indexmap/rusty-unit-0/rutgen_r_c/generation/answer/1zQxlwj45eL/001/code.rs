// Answer 0

#[test]
fn test_shift_insert_within_bounds() {
    struct Indices {
        indices: Vec<usize>,
    }
    
    struct Entries<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    struct OccupiedEntry<'a, K, V> {
        _marker: &'a (K, V),
    }

    let mut indices = Indices { indices: vec![0, 1, 2] };
    let mut entries = Entries { entries: vec![
        Bucket { hash: HashValue(1), key: "a", value: 10 },
        Bucket { hash: HashValue(2), key: "b", value: 20 },
        Bucket { hash: HashValue(3), key: "c", value: 30 },
    ]};

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let vacant_entry = VacantEntry {
        map: ref_mut,
        hash: HashValue(4),
        key: "d",
    };
    
    let value_ref = vacant_entry.shift_insert(1, 40);
    assert_eq!(*value_ref, 40);
    assert_eq!(entries.entries[1].value, 40);
    assert_eq!(entries.entries[2].key, "b");
}

#[should_panic]
fn test_shift_insert_out_of_bounds() {
    struct Indices {
        indices: Vec<usize>,
    }
    
    struct Entries<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    struct OccupiedEntry<'a, K, V> {
        _marker: &'a (K, V),
    }

    let mut indices = Indices { indices: vec![0, 1, 2] };
    let mut entries = Entries { entries: vec![
        Bucket { hash: HashValue(1), key: "a", value: 10 },
        Bucket { hash: HashValue(2), key: "b", value: 20 },
        Bucket { hash: HashValue(3), key: "c", value: 30 },
    ]};

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let vacant_entry = VacantEntry {
        map: ref_mut,
        hash: HashValue(4),
        key: "d",
    };

    vacant_entry.shift_insert(5, 50);
}

