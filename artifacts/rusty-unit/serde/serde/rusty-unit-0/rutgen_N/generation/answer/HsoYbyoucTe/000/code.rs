// Answer 0

#[test]
fn test_expecting() {
    use std::fmt;

    struct MockFormatter {
        output: String,
    }

    impl fmt::Write for MockFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut formatter = MockFormatter { output: String::new() };

    let result = expecting(&(), &mut formatter);
    assert!(result.is_ok());
    assert_eq!(formatter.output, "anything at all");
}

fn expecting(_: &(), formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_str("anything at all")
}

