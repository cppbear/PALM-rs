// Answer 0

#[test]
fn test_get_key_value_mut() {
    struct TestEntries {
        data: Vec<Bucket<i32, String>>,
    }
    
    impl Entries for TestEntries {
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

    let mut entries = TestEntries { data: vec![
        Bucket { hash: HashValue::new(1), key: 42, value: String::from("value1") },
        Bucket { hash: HashValue::new(2), key: 43, value: String::from("value2") },
    ]};
    
    let index = 0;
    let mut occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index,
        hash_builder: PhantomData,
    };

    let (key, value) = occupied_entry.get_key_value_mut();
    
    assert_eq!(*key, 42);
    assert_eq!(*value, "value1");

    *value = String::from("new_value");
    assert_eq!(entries.data[0].value, "new_value");
}

#[test]
fn test_get_key_value_mut_boundary_conditions() {
    struct TestEntries {
        data: Vec<Bucket<i32, String>>,
    }
    
    impl Entries for TestEntries {
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

    let mut entries = TestEntries { data: vec![
        Bucket { hash: HashValue::new(1), key: 42, value: String::from("value1") },
    ]};

    let index = 0;
    let mut occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index,
        hash_builder: PhantomData,
    };

    let (key, value) = occupied_entry.get_key_value_mut();
    
    assert_eq!(*key, 42);
    assert_eq!(*value, "value1");
    
    *value = String::from("boundary_value");
    assert_eq!(entries.data[0].value, "boundary_value");
}

