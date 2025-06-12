// Answer 0

#[test]
fn test_bool_visitor_expecting() {
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
    let visitor = BoolVisitor;

    visitor.expecting(&mut formatter).unwrap();
    assert_eq!(formatter.output, "a boolean");
}

