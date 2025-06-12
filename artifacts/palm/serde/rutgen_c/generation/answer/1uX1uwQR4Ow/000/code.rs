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
    
    let mut mock_formatter = MockFormatter { output: String::new() };
    let visitor = UntaggedUnitVisitor {
        type_name: "TestType",
        variant_name: "TestVariant",
    };
    
    let result = visitor.expecting(&mut mock_formatter);
    
    assert!(result.is_ok());
    assert_eq!(mock_formatter.output, "unit variant TestType::TestVariant");
}

