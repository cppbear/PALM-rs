// Answer 0

#[test]
fn test_key_function() {
    use hashbrown::HashMap;

    struct VacantEntryRef<'a, Q> {
        key: &'a Q,
    }

    impl<'a, Q> VacantEntryRef<'a, Q> {
        pub fn key(&self) -> &'a Q {
            self.key
        }
    }

    let map: HashMap<String, u32> = HashMap::new();
    let key: &str = "poneyland";
    let vacant_entry_ref = VacantEntryRef { key };

    assert_eq!(vacant_entry_ref.key(), "poneyland");
}

#[test]
fn test_key_function_empty() {
    use hashbrown::HashMap;

    struct VacantEntryRef<'a, Q> {
        key: &'a Q,
    }

    impl<'a, Q> VacantEntryRef<'a, Q> {
        pub fn key(&self) -> &'a Q {
            self.key
        }
    }

    let map: HashMap<String, u32> = HashMap::new();
    let key: &str = "";
    let vacant_entry_ref = VacantEntryRef { key };

    assert_eq!(vacant_entry_ref.key(), "");
}

