// Answer 0

#[test]
fn test_unit_visitor_expecting() {
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
    let visitor = UnitVisitor;

    assert!(visitor.expecting(&mut formatter).is_ok());
    assert_eq!(formatter.output, "unit");
}

