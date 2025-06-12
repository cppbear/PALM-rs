// Answer 0

#[test]
fn test_expectation_nothing() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Option<()>;
        
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("option")
        }
    }

    let mut formatter = std::fmt::Formatter::new();
    let visitor = TestVisitor;
    assert!(visitor.expecting(&mut formatter).is_ok());
    let expected_output = "option".to_string();
    assert_eq!(formatter.as_str(), expected_output);
}

#[test]
fn test_expectation_with_value() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Option<()>;
        
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("option")
        }
    }

    let mut formatter = std::fmt::Formatter::new();
    let visitor = TestVisitor;
    visitor.expecting(&mut formatter).unwrap();
    
    assert!(formatter.as_str().contains("option"));
}

