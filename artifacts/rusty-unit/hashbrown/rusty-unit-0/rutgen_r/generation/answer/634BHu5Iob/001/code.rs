// Answer 0

#[test]
fn test_key_with_existing_vacant_entry() {
    use hashbrown::HashMap;

    struct VacantEntryRef<'a, Q> {
        key: &'a Q,
    }

    impl<'a, Q> VacantEntryRef<'a, Q> {
        pub fn key(&self) -> &'a Q {
            self.key
        }
    }

    let key: &str = "poneyland";
    let entry = VacantEntryRef { key };
    
    assert_eq!(entry.key(), "poneyland");
}

#[test]
fn test_key_with_diff_key_type() {
    use hashbrown::HashMap;

    struct VacantEntryRef<'a, Q: std::fmt::Debug> {
        key: &'a Q,
    }

    impl<'a, Q: std::fmt::Debug> VacantEntryRef<'a, Q> {
        pub fn key(&self) -> &'a Q {
            self.key
        }
    }

    let key: &i32 = &42;
    let entry = VacantEntryRef { key };
    
    assert_eq!(*entry.key(), 42);
}

