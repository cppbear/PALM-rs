// Answer 0

#[test]
fn test_fmt() {
    use std::fmt;

    struct RawVacantEntryMut;

    impl fmt::Debug for RawVacantEntryMut {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("RawVacantEntryMut").finish()
        }
    }

    let entry = RawVacantEntryMut;
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", entry);
    
    assert!(result.is_ok());
    assert_eq!(output, "RawVacantEntryMut");
}

