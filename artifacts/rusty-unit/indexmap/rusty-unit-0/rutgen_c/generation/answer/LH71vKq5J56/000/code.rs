// Answer 0

#[test]
fn test_index() {
    struct MockIndices {
        data: Vec<usize>,
    }

    impl MockIndices {
        fn len(&self) -> usize {
            self.data.len()
        }
    }

    struct MockEntries<K, V> {
        _marker: std::marker::PhantomData<(K, V)>,
    }

    struct MockRefMut<'a, K, V> {
        indices: &'a mut MockIndices,
        entries: &'a mut MockEntries<K, V>,
    }

    let mut indices = MockIndices { data: vec![1, 2, 3] };
    let mut entries = MockEntries { _marker: std::marker::PhantomData };
    let ref_mut = MockRefMut { indices: &mut indices, entries: &mut entries };

    let vacant_entry = VacantEntry { map: ref_mut, hash: HashValue(0), key: "test_key" };

    assert_eq!(vacant_entry.index(), 3);
}

#[test]
fn test_index_empty() {
    struct MockIndices {
        data: Vec<usize>,
    }

    impl MockIndices {
        fn len(&self) -> usize {
            self.data.len()
        }
    }

    struct MockEntries<K, V> {
        _marker: std::marker::PhantomData<(K, V)>,
    }

    struct MockRefMut<'a, K, V> {
        indices: &'a mut MockIndices,
        entries: &'a mut MockEntries<K, V>,
    }

    let mut indices = MockIndices { data: vec![] };
    let mut entries = MockEntries { _marker: std::marker::PhantomData };
    let ref_mut = MockRefMut { indices: &mut indices, entries: &mut entries };

    let vacant_entry = VacantEntry { map: ref_mut, hash: HashValue(0), key: "test_key" };

    assert_eq!(vacant_entry.index(), 0);
}

