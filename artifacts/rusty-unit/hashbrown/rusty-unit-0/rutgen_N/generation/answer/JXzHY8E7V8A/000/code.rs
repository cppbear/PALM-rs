// Answer 0

#[test]
fn test_fmt_vacant_entry() {
    struct VacantEntry {
        data: Option<i32>,
    }
    
    impl VacantEntry {
        fn get(&self) -> &Option<i32> {
            &self.data
        }
    }
    
    use std::fmt;
    
    let entry = VacantEntry { data: None };
    let mut output = String::new();
    {
        let mut formatter = fmt::Formatter::new(&mut output);
        entry.fmt(&mut formatter).unwrap();
    }
    
    assert_eq!(output, "VacantEntry(None)");
}

#[test]
fn test_fmt_vacant_entry_with_value() {
    struct VacantEntry {
        data: Option<i32>,
    }
    
    impl VacantEntry {
        fn get(&self) -> &Option<i32> {
            &self.data
        }
    }
    
    use std::fmt;
    
    let entry = VacantEntry { data: Some(42) };
    let mut output = String::new();
    {
        let mut formatter = fmt::Formatter::new(&mut output);
        entry.fmt(&mut formatter).unwrap();
    }
    
    assert_eq!(output, "VacantEntry(Some(42))");
}

