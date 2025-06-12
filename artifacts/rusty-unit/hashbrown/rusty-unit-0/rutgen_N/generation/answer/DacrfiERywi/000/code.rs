// Answer 0

#[test]
fn test_fmt() {
    struct RawEntryBuilder;

    impl std::fmt::Debug for RawEntryBuilder {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("RawEntryBuilder").finish()
        }
    }

    let entry = RawEntryBuilder;
    let result = format!("{:?}", entry);
    assert_eq!(result, "RawEntryBuilder");
}

