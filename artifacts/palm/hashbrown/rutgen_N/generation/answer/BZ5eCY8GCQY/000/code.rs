// Answer 0

#[test]
fn test_fmt_vacant_entry() {
    use std::fmt;

    struct VacantEntry;

    impl fmt::Display for VacantEntry {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("VacantEntry")
        }
    }

    let entry = VacantEntry;
    let result = format!("{}", entry);
    assert_eq!(result, "VacantEntry");
}

