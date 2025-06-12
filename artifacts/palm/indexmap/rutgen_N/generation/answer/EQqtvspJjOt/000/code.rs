// Answer 0

#[test]
fn test_fmt_raw_entry_builder() {
    use std::fmt;

    struct RawEntryBuilder;

    impl RawEntryBuilder {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("RawEntryBuilder").finish_non_exhaustive()
        }
    }

    let entry_builder = RawEntryBuilder;
    let mut output = String::new();
    {
        let formatter = &mut fmt::Formatter::new(&mut output);
        entry_builder.fmt(formatter).unwrap();
    }

    assert!(output.contains("RawEntryBuilder"));
}

