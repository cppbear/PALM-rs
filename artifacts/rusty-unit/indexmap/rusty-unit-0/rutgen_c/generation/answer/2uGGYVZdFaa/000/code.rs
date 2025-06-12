// Answer 0

#[test]
fn test_shift_remove_entry() {
    use hashbrown::hash_table;
    use crate::{Equivalent, HashValue, IndexMap};

    struct TestEntries {
        data: Vec<Bucket<u32, String>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<u32, String>;

        fn into_entries(self) -> Vec<Self::Entry> {
            self.data
        }

        fn as_entries(&self) -> &[Self::Entry] {
            &self.data
        }

        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut self.data
        }

        fn with_entries<F>(&mut self, f: F)
        where
            F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.data)
        }
    }

    let mut entries = TestEntries {
        data: vec![
            Bucket { hash: HashValue::default(), key: 1, value: "one".to_string() },
            Bucket { hash: HashValue::default(), key: 2, value: "two".to_string() },
            Bucket { hash: HashValue::default(), key: 3, value: "three".to_string() },
        ],
    };

    // Simulate the occupied entry with an index pointing to the second entry.
    let index_entry = hash_table::OccupiedEntry::from_index(1);

    let raw_occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: index_entry,
        hash_builder: PhantomData,
    };

    let (removed_key, removed_value) = raw_occupied_entry.shift_remove_entry();

    assert_eq!(removed_key, 2);
    assert_eq!(removed_value, "two");

    // Verify the remaining entries after removal
    assert_eq!(entries.as_entries().len(), 2);
    assert_eq!(entries.as_entries()[0].key, 1);
    assert_eq!(entries.as_entries()[0].value, "one");
    assert_eq!(entries.as_entries()[1].key, 3);
    assert_eq!(entries.as_entries()[1].value, "three");
} 

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_shift_remove_entry_out_of_bounds() {
    use hashbrown::hash_table;
    use crate::{Equivalent, HashValue, IndexMap};

    struct TestEntries {
        data: Vec<Bucket<u32, String>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<u32, String>;

        fn into_entries(self) -> Vec<Self::Entry> {
            self.data
        }

        fn as_entries(&self) -> &[Self::Entry] {
            &self.data
        }

        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut self.data
        }

        fn with_entries<F>(&mut self, f: F)
        where
            F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.data)
        }
    }

    let mut entries = TestEntries {
        data: vec![
            Bucket { hash: HashValue::default(), key: 1, value: "one".to_string() },
        ],
    };

    // Simulate an occupied entry with an index pointing outside of bounds.
    let index_entry = hash_table::OccupiedEntry::from_index(1); // Out of bounds

    let raw_occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: index_entry,
        hash_builder: PhantomData,
    };

    // This should panic since the index is out of bounds
    let _ = raw_occupied_entry.shift_remove_entry();
}

