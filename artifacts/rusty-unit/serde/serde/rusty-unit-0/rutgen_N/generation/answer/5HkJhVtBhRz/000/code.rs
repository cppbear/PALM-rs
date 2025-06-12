// Answer 0

#[test]
fn test_expecting() {
    use std::fmt;

    struct Expecting {
        expecting: &'static str,
    }

    impl Expecting {
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str(self.expecting)
        }
    }

    let my_expectation = Expecting {
        expecting: "This is the expected value.",
    };

    let mut result = String::new();
    let formatter = &mut fmt::Formatter::new(&mut result);
    let fmt_result = my_expectation.expecting(formatter);

    assert!(fmt_result.is_ok());
    assert_eq!(result, "This is the expected value.");
}

