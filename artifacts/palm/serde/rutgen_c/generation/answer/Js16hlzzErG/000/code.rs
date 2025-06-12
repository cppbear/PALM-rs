// Answer 0

#[test]
fn test_visit_bool() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = bool;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a boolean value")
        }
        
        fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(v)
        }
    }

    let visitor = TestVisitor;
    
    assert_eq!(visitor.visit_bool(true).unwrap(), true);
    assert_eq!(visitor.visit_bool(false).unwrap(), false);
}

#[test]
#[should_panic]
fn test_visit_bool_invalid() {
    struct InvalidVisitor;

    impl<'de> Visitor<'de> for InvalidVisitor {
        type Value = bool;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a boolean value")
        }

        fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
        where
            E: Error,
        {
            if !v {
                panic!("Expected true");
            }
            Ok(v)
        }
    }

    let visitor = InvalidVisitor;
    
    visitor.visit_bool(false).unwrap(); // This will panic
}

