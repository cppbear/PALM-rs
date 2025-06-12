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
    let ignored_any = IgnoredAny;

    let result = ignored_any.expecting(&mut formatter);
    assert!(result.is_ok());
    assert_eq!(formatter.output, "anything at all");
}

