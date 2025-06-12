// Answer 0

#[test]
fn test_or_default_with_vacant_entry() {
    struct TestEntry {
        value: Option<i32>,
    }

    impl Default for TestEntry {
        fn default() -> Self {
            TestEntry { value: Some(42) }
        }
    }

    enum Entry<'a> {
        Vacant(&'a mut TestEntry),
        Occupied(&'a mut TestEntry),
    }

    let mut entry = TestEntry { value: None };
    let mut vacant_entry = Entry::Vacant(&mut entry);
    
    let value_ref = match vacant_entry {
        Entry::Occupied(_) => unreachable!(),
        Entry::Vacant(ref mut e) => e.insert(TestEntry::default()),
    };

    assert_eq!(value_ref.value, Some(42));
}

#[test]
fn test_or_default_with_occupied_entry() {
    struct TestEntry {
        value: Option<i32>,
    }

    impl Default for TestEntry {
        fn default() -> Self {
            TestEntry { value: Some(99) }
        }
    }

    enum Entry<'a> {
        Vacant(&'a mut TestEntry),
        Occupied(&'a mut TestEntry),
    }

    let mut entry = TestEntry { value: Some(20) };
    let mut occupied_entry = Entry::Occupied(&mut entry);
    
    let value_ref = match occupied_entry {
        Entry::Occupied(ref mut e) => e,
        Entry::Vacant(_) => unreachable!(),
    };

    *value_ref = TestEntry::default();
    
    assert_eq!(entry.value, Some(99));
}

