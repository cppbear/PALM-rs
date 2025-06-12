// Answer 0

#[test]
fn test_expecting_returns_ok_for_unit() {
    use std::fmt::Formatter;
    
    struct MockFormatter {
        output: String,
    }
    
    impl Formatter for MockFormatter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }
    
    let mut formatter = MockFormatter { output: String::new() };
    let visitor = UnitVisitor;
    
    let result = visitor.expecting(&mut formatter);
    
    assert!(result.is_ok());
    assert_eq!(formatter.output, "unit");
}

#[test]
fn test_expecting_with_empty_formatter() {
    use std::fmt::Formatter;
    
    struct MockFormatter {
        output: String,
    }
    
    impl Formatter for MockFormatter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }
    
    let mut formatter = MockFormatter { output: String::new() };
    let visitor = UnitVisitor;
    
    visitor.expecting(&mut formatter).unwrap();
    
    assert_eq!(formatter.output, "unit");
}

