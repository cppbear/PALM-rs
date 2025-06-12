// Answer 0

#[test]
fn test_into_key_valid() {
    struct TestEntries<'a> {
        data: Vec<Bucket<i32, String>>,
        marker: PhantomData<&'a ()>,
    }

    impl<'a> Entries for TestEntries<'a> {
        type Entry = Bucket<i32, String>;

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
            F: FnOnce(&mut [Self::Entry]),
        {
            f(&mut self.data);
        }
    }

    let mut entries = TestEntries {
        data: vec![Bucket { hash: HashValue::default(), key: 1, value: "value1".to_string() }],
        marker: PhantomData,
    };

    let index = 0;
    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::new(index),
        hash_builder: PhantomData,
    };

    let key_ref: &mut i32 = occupied_entry.into_key();
    assert_eq!(*key_ref, 1);
    *key_ref = 10;
    assert_eq!(entries.as_entries()[index].key, 10);
}

#[should_panic]
fn test_into_key_out_of_bounds() {
    struct TestEntries<'a> {
        data: Vec<Bucket<i32, String>>,
        marker: PhantomData<&'a ()>,
    }

    impl<'a> Entries for TestEntries<'a> {
        type Entry = Bucket<i32, String>;

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
            F: FnOnce(&mut [Self::Entry]),
        {
            f(&mut self.data);
        }
    }

    let mut entries = TestEntries {
        data: vec![],
        marker: PhantomData,
    };

    let index = 0;
    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::new(index),
        hash_builder: PhantomData,
    };

    let _key_ref: &mut i32 = occupied_entry.into_key();
}

