// Answer 0

#[test]
fn test_visiting_expecting() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string")
        }

        // Omit other methods to focus on testing `expecting`
        fn visit_str<E>(self, _v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(())
        }
    }
    
    let visitor = TestVisitor;
    let mut buffer = String::new();
    let result = visitor.expecting(&mut buffer);

    assert!(result.is_ok());
    assert_eq!(buffer, "a string");
}

