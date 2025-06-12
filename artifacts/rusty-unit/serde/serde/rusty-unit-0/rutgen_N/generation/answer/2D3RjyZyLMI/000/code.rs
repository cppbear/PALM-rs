// Answer 0

#[cfg(test)]
mod tests {
    use std::fmt;

    struct TestStruct;

    impl TestStruct {
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("os string")
        }
    }

    #[test]
    fn test_expectation() {
        let test_instance = TestStruct;
        let mut output = Vec::new();
        let mut formatter = fmt::Formatter::new(&mut output);
        
        let result = test_instance.expecting(&mut formatter);
        
        assert!(result.is_ok());
        assert_eq!(String::from_utf8(output).unwrap(), "os string");
    }
}

