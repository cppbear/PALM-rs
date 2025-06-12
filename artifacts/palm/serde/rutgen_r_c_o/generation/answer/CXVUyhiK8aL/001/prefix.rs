// Answer 0

#[test]
fn test_string_visitor_expecting() {
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
    let visitor = StringVisitor;

    visitor.expecting(&mut formatter).unwrap();
    
    // Function executed successfully, no assertion here
}

#[test]
#[should_panic]
fn test_string_visitor_expecting_panic() {
    struct MockFormatter;

    impl fmt::Write for MockFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            panic!("Intentional panic for testing");
        }
    }

    let mut formatter = MockFormatter;
    let visitor = StringVisitor;

    visitor.expecting(&mut formatter).unwrap();

    // Function produces a panic, no assertion here
}

