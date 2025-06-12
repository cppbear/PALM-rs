// Answer 0

#[test]
fn test_expecting() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("option")
        }
    }

    let mut output = String::new();
    let result = TestVisitor.expecting(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "option");
}

#[test]
fn test_expecting_empty_string() {
    struct EmptyVisitor;

    impl<'de> Visitor<'de> for EmptyVisitor {
        type Value = ();
        
        fn expecting(&self, _formatter: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }
    }

    let mut output = String::new();
    let visitor = EmptyVisitor;
    let result = visitor.expecting(&mut output);
    
    assert!(result.is_ok());
    assert!(output.is_empty());
}

