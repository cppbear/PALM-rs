// Answer 0

#[test]
fn test_fmt_raw_entry_builder() {
    use std::fmt;

    struct RawEntryBuilder;

    impl fmt::Debug for RawEntryBuilder {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("RawEntryBuilder").finish()
        }
    }

    let builder = RawEntryBuilder;

    let mut output = String::new();
    {
        let formatter = &mut fmt::Formatter::new(&mut output);
        builder.fmt(formatter).expect("Formatting failed");
    }

    assert_eq!(output, "RawEntryBuilder");
}

