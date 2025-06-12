// Answer 0

#[test]
fn test_insert() {
    struct TestEntries {
        data: Vec<(usize, String)>,
    }

    impl Entries for TestEntries {
        type Entry = (usize, String);
        
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
            F: FnOnce(&mut [Self::Entry]) 
        {
            f(&mut self.data);
        }
    }

    // Setup
    let mut entries = TestEntries { data: vec![(0, String::from("value0"))] };
    let index_entry = hashbrown::hash_table::OccupiedEntry::from_entry(&mut entries.data[0]);
    let mut raw_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: index_entry,
        hash_builder: PhantomData,
    };

    // Test initial value
    let old_value = raw_entry.insert(String::from("new_value"));
    assert_eq!(old_value, "value0");
    assert_eq!(raw_entry.get(), "new_value");

    // Test inserting a new value: boundary condition (empty entry)
    entries.data.push((1, String::from("old_value1")));
    let index_entry = hashbrown::hash_table::OccupiedEntry::from_entry(&mut entries.data[1]);
    let mut raw_entry_empty = RawOccupiedEntryMut {
        entries: &mut entries,
        index: index_entry,
        hash_builder: PhantomData,
    };

    let old_value_empty = raw_entry_empty.insert(String::from("new_value1"));
    assert_eq!(old_value_empty, "old_value1");
    assert_eq!(raw_entry_empty.get(), "new_value1");
}

