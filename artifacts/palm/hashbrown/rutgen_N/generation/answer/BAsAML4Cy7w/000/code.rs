// Answer 0

#[test]
fn test_fmt_vacant_entry() {
    use std::fmt;

    struct VacantEntry<K> {
        key: K,
    }

    impl<K: fmt::Debug> VacantEntry<K> {
        fn key(&self) -> &K {
            &self.key
        }
    }

    impl<K: fmt::Debug> fmt::Debug for VacantEntry<K> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_tuple("VacantEntry").field(self.key()).finish()
        }
    }

    let entry = VacantEntry { key: "test_key" };
    let mut buffer = String::new();
    let formatter = &mut fmt::Formatter::new(&mut buffer);
    entry.fmt(formatter).unwrap();

    assert_eq!(buffer, "VacantEntry(\"test_key\")");
}

