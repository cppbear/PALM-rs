// Answer 0

#[test]
fn test_into_muts() {
    struct MockEntries<K, V> {
        buckets: Vec<Bucket<K, V>>,
    }

    impl<K, V> super::Entries for MockEntries<K, V> {
        type Entry = Bucket<K, V>;

        fn into_entries(self) -> Vec<Self::Entry> {
            self.buckets
        }

        fn as_entries(&self) -> &[Self::Entry] {
            &self.buckets
        }

        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut self.buckets
        }

        fn with_entries<F>(&mut self, f: F)
        where
            F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.buckets);
        }
    }

    let mut entries = MockEntries {
        buckets: vec![
            Bucket { hash: 1.into(), key: "key1", value: 10 },
            Bucket { hash: 2.into(), key: "key2", value: 20 },
        ],
    };

    let index = 0;
    let occupied_entry = OccupiedEntry::new(&mut entries, hashbrown::hash_table::OccupiedEntry::new(index));

    let (key_mut, value_mut) = occupied_entry.into_muts();
    
    assert_eq!(*key_mut, "key1");
    assert_eq!(*value_mut, 10);
}

#[test]
#[should_panic]
fn test_into_muts_out_of_bounds() {
    struct MockEntries<K, V> {
        buckets: Vec<Bucket<K, V>>,
    }

    impl<K, V> super::Entries for MockEntries<K, V> {
        type Entry = Bucket<K, V>;

        fn into_entries(self) -> Vec<Self::Entry> {
            self.buckets
        }

        fn as_entries(&self) -> &[Self::Entry] {
            &self.buckets
        }

        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut self.buckets
        }

        fn with_entries<F>(&mut self, f: F)
        where
            F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.buckets);
        }
    }

    let mut entries = MockEntries {
        buckets: vec![
            Bucket { hash: 1.into(), key: "key1", value: 10 },
        ],
    };

    let index = 1; // Out of bounds
    let occupied_entry = OccupiedEntry::new(&mut entries, hashbrown::hash_table::OccupiedEntry::new(index));

    // This should panic since index is out of bounds
    let _ = occupied_entry.into_muts();
}

