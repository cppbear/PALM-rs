// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt;

    struct TestStruct(fmt::Formatter<'static>);

    impl fmt::Display for TestStruct {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "TestStruct")
        }
    }

    #[test]
    fn test_fmt() {
        let mut output = String::new();
        let formatter = &mut fmt::Formatter::new(&mut output);
        
        let test_struct = TestStruct(fmt::Formatter::new(formatter));
        assert!(write!(formatter, "{}", test_struct).is_ok());
        assert_eq!(output, "TestStruct");
    }
}

