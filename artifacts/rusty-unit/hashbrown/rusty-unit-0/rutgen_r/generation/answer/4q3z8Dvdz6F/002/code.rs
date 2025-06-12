// Answer 0

#[test]
fn test_and_modify_with_occupied_entry() {
    use hashbrown::HashMap;
    use hashbrown::raw::RawEntryMut;

    struct TestEntry<'a> {
        key: &'a str,
        value: u32,
    }

    struct TestRawEntryMut<'a> {
        entry: Option<TestEntry<'a>>,
    }

    impl<'a> TestRawEntryMut<'a> {
        fn from_key(key: &'a str) -> Self {
            Self {
                entry: Some(TestEntry { key, value: 10 }),
            }
        }

        fn and_modify<F>(self, f: F) -> Self
        where
            F: FnOnce(&mut &'a str, &mut u32),
        {
            if let Some(mut entry) = self.entry {
                f(&mut entry.key, &mut entry.value);
                RawEntryMut::Occupied(Some(entry)) // Mimicking the expected output.
            } else {
                RawEntryMut::Vacant // This part is not reachable due to test setup.
            }
        }
    }

    let mut map: HashMap<&str, u32> = HashMap::new();
    
    map.raw_entry_mut().from_key("test_key").and_modify(|_k, v| {
        *v += 20;
    });

    assert_eq!(map["test_key"], 30);

    map.raw_entry_mut().from_key("test_key").and_modify(|_k, v| {
        *v += 10;
    });

    assert_eq!(map["test_key"], 40);
}

#[should_panic]
fn test_and_modify_with_vacant_entry() {
    use hashbrown::HashMap;
    use hashbrown::raw::RawEntryMut;

    struct TestEntry<'a> {
        key: &'a str,
        value: u32,
    }

    struct TestRawEntryMut<'a> {
        entry: Option<TestEntry<'a>>,
    }

    impl<'a> TestRawEntryMut<'a> {
        fn from_key(key: &'a str) -> Self {
            Self { entry: None }
        }

        fn and_modify<F>(self, _f: F) -> Self {
            // No entry to modify causes panic
            assert!(self.entry.is_some()); // This assertion should panic in the test
            self
        }
    }

    let mut map: HashMap<&str, u32> = HashMap::new();

    map.raw_entry_mut().from_key("vacant_key").and_modify(|_k, _v| {
        // Shouldn't reach here as the entry is vacant
    });
}

