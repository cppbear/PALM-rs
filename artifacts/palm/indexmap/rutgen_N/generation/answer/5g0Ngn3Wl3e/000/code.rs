// Answer 0

#[test]
fn test_fmt_raw_entry_builder_mut() {
    use std::fmt;

    struct RawEntryBuilderMut;

    impl RawEntryBuilderMut {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("RawEntryBuilderMut").finish_non_exhaustive()
        }
    }

    let entry = RawEntryBuilderMut;
    let mut buffer = String::new();
    let result = entry.fmt(&mut fmt::Formatter::new(&mut buffer));

    assert!(result.is_ok());
    assert!(buffer.contains("RawEntryBuilderMut"));
}

