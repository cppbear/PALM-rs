// Answer 0

#[test]
fn test_expecting() {
    use std::fmt;

    struct EmptyArrayFormatter;

    impl fmt::Display for EmptyArrayFormatter {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.expecting(f)
        }
    }

    impl EmptyArrayFormatter {
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an empty array")
        }
    }

    let mut output = String::new();
    let formatter = &mut fmt::Formatter::new(&mut output);
    let empty_array_formatter = EmptyArrayFormatter;
    
    assert!(empty_array_formatter.expecting(formatter).is_ok());
    assert_eq!(output, "an empty array");
}

