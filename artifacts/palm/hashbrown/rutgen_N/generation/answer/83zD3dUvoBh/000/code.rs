// Answer 0

#[test]
fn test_vacant_entry_ref_fmt() {
    use std::fmt;

    struct VacantEntryRef {
        key: String,
    }

    impl VacantEntryRef {
        fn new(key: &str) -> Self {
            VacantEntryRef {
                key: key.to_string(),
            }
        }

        fn key(&self) -> &str {
            &self.key
        }
    }

    impl fmt::Debug for VacantEntryRef {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_tuple("VacantEntryRef").field(&self.key()).finish()
        }
    }

    let entry = VacantEntryRef::new("test_key");
    let result = format!("{:?}", entry);
    assert_eq!(result, "VacantEntryRef(\"test_key\")");
}

