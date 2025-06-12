// Answer 0

#[test]
fn test_fmt_occupied_entry() {
    use std::fmt;

    struct OccupiedEntry {
        value: i32,
    }

    impl OccupiedEntry {
        fn get(&self) -> &i32 {
            &self.value
        }
        
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("OccupiedEntry")
                .field("value", self.get())
                .finish()
        }
    }

    let entry = OccupiedEntry { value: 42 };
    let mut output = String::new();
    {
        let formatter = &mut fmt::Formatter::new(&mut output);
        entry.fmt(formatter).unwrap();
    }
    
    assert_eq!(output, "OccupiedEntry { value: 42 }");
}

#[test]
fn test_fmt_occupied_entry_zero() {
    use std::fmt;

    struct OccupiedEntry {
        value: i32,
    }

    impl OccupiedEntry {
        fn get(&self) -> &i32 {
            &self.value
        }
        
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("OccupiedEntry")
                .field("value", self.get())
                .finish()
        }
    }

    let entry = OccupiedEntry { value: 0 };
    let mut output = String::new();
    {
        let formatter = &mut fmt::Formatter::new(&mut output);
        entry.fmt(formatter).unwrap();
    }
    
    assert_eq!(output, "OccupiedEntry { value: 0 }");
}

#[test]
fn test_fmt_occupied_entry_negative() {
    use std::fmt;

    struct OccupiedEntry {
        value: i32,
    }

    impl OccupiedEntry {
        fn get(&self) -> &i32 {
            &self.value
        }
        
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("OccupiedEntry")
                .field("value", self.get())
                .finish()
        }
    }

    let entry = OccupiedEntry { value: -10 };
    let mut output = String::new();
    {
        let formatter = &mut fmt::Formatter::new(&mut output);
        entry.fmt(formatter).unwrap();
    }
    
    assert_eq!(output, "OccupiedEntry { value: -10 }");
}

