// Answer 0

#[cfg(test)]
mod tests {
    use std::fmt;

    struct TestExpecting<'a> {
        expecting: &'a str,
    }

    impl<'a> TestExpecting<'a> {
        fn new(expect: &'a str) -> Self {
            TestExpecting { expecting: expect }
        }

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str(self.expecting)
        }
    }

    #[test]
    fn test_expectation_non_empty() {
        let test_value = TestExpecting::new("This is a test expecting string.");
        let mut output = String::new();
        let result = test_value.expecting(&mut fmt::Formatter::new(&mut output));

        assert_eq!(result, Ok(()));
        assert_eq!(output, "This is a test expecting string.");
    }

    #[test]
    fn test_expectation_empty() {
        let test_value = TestExpecting::new("");
        let mut output = String::new();
        let result = test_value.expecting(&mut fmt::Formatter::new(&mut output));

        assert_eq!(result, Ok(()));
        assert_eq!(output, "");
    }

    #[test]
    fn test_expectation_long_string() {
        let long_expectation = "x".repeat(1000);
        let test_value = TestExpecting::new(&long_expectation);
        let mut output = String::new();
        let result = test_value.expecting(&mut fmt::Formatter::new(&mut output));

        assert_eq!(result, Ok(()));
        assert_eq!(output, long_expectation);
    }

    #[should_panic]
    #[test]
    fn test_expectation_panic() {
        let test_value = TestExpecting::new("This should not panic.");
        let mut formatter = fmt::Formatter::new(&mut String::new());
        // Here we deliberately try to put it in a bad state to trigger a panic
        formatter.write_str("");
        let _ = test_value.expecting(&mut formatter);
    }
}

