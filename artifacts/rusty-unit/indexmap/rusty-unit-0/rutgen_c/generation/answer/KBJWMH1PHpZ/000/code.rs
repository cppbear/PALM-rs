// Answer 0

#[test]
fn test_index() {
    struct Indices {
        data: Vec<usize>,
    }

    struct Entries<K, V> {
        _phantom: PhantomData<(K, V)>,
    }

    let mut indices = Indices { data: vec![0, 1, 2] };
    let entries = Entries { _phantom: PhantomData };
    
    let ref_mut = RefMut {
        indices: &mut indices,
        entries: &mut entries,
    };

    let raw_entry = RawVacantEntryMut {
        map: ref_mut,
        hash_builder: &(),
    };

    assert_eq!(raw_entry.index(), 3);
}

