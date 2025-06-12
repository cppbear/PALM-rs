// Answer 0

#[test]
fn test_vacant_entry_fmt() {
    use std::fmt;

    struct VacantEntry<'a, K> {
        key: &'a K,
    }

    impl<'a, K: fmt::Debug> VacantEntry<'a, K> {
        fn key(&self) -> &K {
            self.key
        }
        
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_tuple("VacantEntry").field(self.key()).finish()
        }
    }

    let key = "test_key";
    let vacant_entry = VacantEntry { key: &key };
    
    let mut buffer = String::new();
    let result = vacant_entry.fmt(&mut fmt::Formatter::new(&mut buffer));

    assert!(result.is_ok());
    assert_eq!(buffer, "VacantEntry(\"test_key\")");
}

