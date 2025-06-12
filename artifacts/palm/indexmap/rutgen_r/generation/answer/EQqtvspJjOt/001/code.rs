// Answer 0

#[test]
fn test_fmt_raw_entry_builder() {
    struct RawEntryBuilder;

    impl std::fmt::Debug for RawEntryBuilder {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("RawEntryBuilder").finish_non_exhaustive()
        }
    }

    let entry_builder = RawEntryBuilder;
    let mut output = String::new();
    {
        let mut formatter = std::fmt::Formatter::new(&mut output);
        entry_builder.fmt(&mut formatter).unwrap();
    }

    assert_eq!(output, "RawEntryBuilder");
}

