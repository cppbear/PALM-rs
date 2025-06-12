// Answer 0

#[test]
fn test_do_deserialize_u128_valid() {
    struct TestVisitor;

    impl<'any> de::Visitor<'any> for TestVisitor {
        type Value = u128;

        fn visit_u128<E>(self, value: u128) -> Result<Self::Value, E> {
            Ok(value)
        }
    }

    let mut deserializer = Deserializer::new("  12345 ");
    
    assert_eq!(deserializer.do_deserialize_u128(TestVisitor), Ok(12345));
}

#[test]
fn test_do_deserialize_u128_negative_number() {
    struct TestVisitor;

    impl<'any> de::Visitor<'any> for TestVisitor {
        type Value = u128;

        fn visit_u128<E>(self, _: u128) -> Result<Self::Value, E> {
            Ok(0)  // This should never be called
        }
    }

    let mut deserializer = Deserializer::new(" -12345 ");
    
    assert!(matches!(deserializer.do_deserialize_u128(TestVisitor), Err(_)));
}

#[test]
fn test_do_deserialize_u128_eof_error() {
    struct TestVisitor;

    impl<'any> de::Visitor<'any> for TestVisitor {
        type Value = u128;

        fn visit_u128<E>(self, _: u128) -> Result<Self::Value, E> {
            Ok(0)  // This should never be called
        }
    }

    let mut deserializer = Deserializer::new(" ");
    
    assert!(matches!(deserializer.do_deserialize_u128(TestVisitor), Err(_)));
}

#[test]
fn test_do_deserialize_u128_parse_error() {
    struct TestVisitor;

    impl<'any> de::Visitor<'any> for TestVisitor {
        type Value = u128;

        fn visit_u128<E>(self, _: u128) -> Result<Self::Value, E> {
            Ok(0)  // This should never be called
        }
    }

    let mut deserializer = Deserializer::new("not_a_number");
    
    assert!(matches!(deserializer.do_deserialize_u128(TestVisitor), Err(_)));
}

