// Answer 0

#[test]
fn test_visit_string() {
    struct TestVisitor;
    
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;
        
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string")
        }
        
        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(From::from(v))
        }
    }

    let visitor = TestVisitor;
    let result: Result<String, ()> = visitor.visit_string("test string".to_string());
    
    assert_eq!(result.unwrap(), "test string");
}

#[test]
fn test_visit_string_empty() {
    struct TestVisitor;
    
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;
        
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string")
        }
        
        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(From::from(v))
        }
    }

    let visitor = TestVisitor;
    let result: Result<String, ()> = visitor.visit_string("".to_string());
    
    assert_eq!(result.unwrap(), "");
}

