// Answer 0

#[test]
fn test_fmt_raw_vacant_entry_mut() {
    use std::fmt;

    struct RawVacantEntryMut;

    impl RawVacantEntryMut {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("RawVacantEntryMut").finish_non_exhaustive()
        }
    }

    let entry = RawVacantEntryMut;
    let mut buffer = String::new();
    let result = entry.fmt(&mut fmt::Formatter::new(&mut buffer));
    assert!(result.is_ok());
}

