// Answer 0

#[test]
fn test_expecting() {
    struct TestVisitor;
    
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("os string")
        }
    }
    
    let mut buf = String::new();
    let mut formatter = fmt::Formatter::new(&mut buf);
    let visitor = TestVisitor;
    
    let result = visitor.expecting(&mut formatter);
    
    assert!(result.is_ok());
    assert_eq!(buf, "os string");
}

