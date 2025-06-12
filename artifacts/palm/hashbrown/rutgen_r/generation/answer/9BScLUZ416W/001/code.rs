// Answer 0

#[test]
fn test_fmt_raw_entry_builder() {
    struct RawEntryBuilder;

    impl std::fmt::Debug for RawEntryBuilder {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("RawEntryBuilder").finish()
        }
    }

    let builder = RawEntryBuilder;
    let mut output = String::new();
    let result = std::fmt::write(&mut output, format_args!("{:?}", builder));

    assert!(result.is_ok());
    assert_eq!(output, "RawEntryBuilder");
}

