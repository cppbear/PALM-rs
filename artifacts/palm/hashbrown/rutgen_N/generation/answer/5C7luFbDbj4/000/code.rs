// Answer 0

#[test]
fn test_fmt_occupied_entry() {
    use std::fmt;

    struct OccupiedEntry<T> {
        value: T,
    }

    impl<T: fmt::Debug> OccupiedEntry<T> {
        fn get(&self) -> &T {
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
    let result = fmt::Formatter::write_str(&mut output, &format!("{:?}", entry));

    assert!(result.is_ok());
    assert!(output.contains("OccupiedEntry"));
    assert!(output.contains("value: 42"));
}

