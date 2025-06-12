// Answer 0

#[test]
fn test_and_modify_vacant_entry() {
    use hashbrown::HashMap;
    use hashbrown::raw_entry::RawEntryMut;

    struct TestEntry {
        map: HashMap<&'static str, u32>,
    }

    impl TestEntry {
        fn new() -> Self {
            TestEntry {
                map: HashMap::new(),
            }
        }

        fn test_modify(&mut self) -> RawEntryMut<'_, &'static str, u32> {
            self.map.raw_entry_mut().from_key("empty_key")
                .and_modify(|_k, _v| { panic!("This should not be called since the entry is vacant"); })
        }
    }

    let mut entry = TestEntry::new();
    let vacant_entry = entry.test_modify();
    
    match vacant_entry {
        RawEntryMut::Vacant(_) => (),
        _ => panic!("Expected a Vacant RawEntryMut"),
    }
}

