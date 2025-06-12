// Answer 0

#[test]
fn test_expecting() {
    use std::fmt;

    struct TestFormatter {
        output: String,
    }

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut formatter = TestFormatter { output: String::new() };
    let result = expecting(&(), &mut formatter);
    
    assert_eq!(result, Ok(()));
    assert_eq!(formatter.output, "a boolean");
}

