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
    let mut output = String::new();
    let result = write!(&mut output, "{}", entry);
    
    assert!(result.is_ok());
    assert_eq!(output, "VacantEntry");
} 

#[test]
fn test_fmt_vacant_entry_with_buffer() {
    use std::fmt;

    struct VacantEntry;

    impl fmt::Display for VacantEntry {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("VacantEntry")
        }
    }

    let entry = VacantEntry;
    let result = entry.to_string();
    
    assert_eq!(result, "VacantEntry");
}

