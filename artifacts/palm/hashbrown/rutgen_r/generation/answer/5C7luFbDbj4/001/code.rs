// Answer 0

#[test]
fn test_fmt_occupied_entry() {
    use std::fmt;

    struct OccupiedEntry {
        value: String,
    }

    impl OccupiedEntry {
        fn get(&self) -> &String {
            &self.value
        }
    }

    impl fmt::Debug for OccupiedEntry {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("OccupiedEntry")
                .field("value", self.get())
                .finish()
        }
    }

    let entry = OccupiedEntry {
        value: String::from("test_value"),
    };

    let result = format!("{:?}", entry);
    assert_eq!(result, "OccupiedEntry { value: \"test_value\" }");
}

#[test]
fn test_fmt_occupied_entry_empty_value() {
    use std::fmt;

    struct OccupiedEntry {
        value: String,
    }

    impl OccupiedEntry {
        fn get(&self) -> &String {
            &self.value
        }
    }

    impl fmt::Debug for OccupiedEntry {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("OccupiedEntry")
                .field("value", self.get())
                .finish()
        }
    }

    let entry = OccupiedEntry {
        value: String::new(),
    };

    let result = format!("{:?}", entry);
    assert_eq!(result, "OccupiedEntry { value: \"\" }");
}

