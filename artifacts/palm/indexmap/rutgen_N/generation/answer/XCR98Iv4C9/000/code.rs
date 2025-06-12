// Answer 0

#[test]
fn test_fmt_vacant_entry() {
    use std::fmt;

    struct VacantEntry<'a> {
        key: &'a str,
    }

    impl<'a> VacantEntry<'a> {
        fn key(&self) -> &str {
            self.key
        }

        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_tuple("VacantEntry").field(self.key()).finish()
        }
    }

    let entry = VacantEntry { key: "test_key" };
    let mut buffer = String::new();
    let result = fmt::Formatter::write(&mut buffer, |f| entry.fmt(f));

    assert!(result.is_ok());
    assert_eq!(buffer, "VacantEntry(\"test_key\")");
}

