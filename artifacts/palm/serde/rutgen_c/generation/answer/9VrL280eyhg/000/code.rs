// Answer 0

#[test]
fn test_expecting() {
    struct TestVisitor;
    
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        
        fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            fmt.write_str("any value")
        }
    }

    let mut output = Vec::new();
    let result = {
        let mut formatter = crate::format::Formatter::new(&mut output);
        TestVisitor.expecting(&TestVisitor, &mut formatter)
    };
    
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "any value");
}

