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

    let raw_entry_builder = RawEntryBuilderMut;

    let result = std::panic::catch_unwind(|| {
        let mut output = String::new();
        let formatter = &mut fmt::Formatter::new(&mut output);
        raw_entry_builder.fmt(formatter)
    });

    assert!(result.is_ok());
}

#[test]
fn test_fmt_raw_entry_builder_mut_for_non_exhaustive() {
    use std::fmt;

    struct RawEntryBuilderMut;

    impl RawEntryBuilderMut {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("RawEntryBuilderMut").finish_non_exhaustive()
        }
    }

    let raw_entry_builder = RawEntryBuilderMut;

    let result = std::panic::catch_unwind(|| {
        let mut output = String::new();
        let formatter = &mut fmt::Formatter::new(&mut output);
        raw_entry_builder.fmt(formatter)
    });

    assert!(result.is_ok());

    let formatted_result = format!("{:?}", raw_entry_builder);
    assert!(formatted_result.contains("RawEntryBuilderMut"));
}

