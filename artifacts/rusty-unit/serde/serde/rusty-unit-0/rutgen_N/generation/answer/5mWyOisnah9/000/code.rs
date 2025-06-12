// Answer 0

#[test]
fn test_expecting() {
    struct DummyFormatter {
        output: String,
    }

    impl fmt::Write for DummyFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }
    
    let mut formatter = DummyFormatter { output: String::new() };
    let result = expecting(&(), &mut formatter);
    
    assert!(result.is_ok());
    assert_eq!(formatter.output, "a boolean");
}

