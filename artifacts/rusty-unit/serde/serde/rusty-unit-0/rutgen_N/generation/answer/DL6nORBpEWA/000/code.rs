// Answer 0

#[test]
fn test_expecting() {
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
    assert_eq!(formatter.output, "a borrowed byte array");
}

