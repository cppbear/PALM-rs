// Answer 0

#[test]
fn test_visit_u64() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = u64;

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
            assert_eq!(v, 42); // Expecting value to be 42
            Ok(v)
        }

        forward_to_deserialize_any! {
            i64, f64, string, bytes, seq, map, struct, unit, unit_struct, newtype_struct,
            tuple, tuple_struct, option, identifier, ...
        }
    }

    let parser_number = ParserNumber::U64(42);
    let result = parser_number.visit(TestVisitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn test_visit_f64() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = f64;

        fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E> {
            assert_eq!(v, 3.14); // Expecting value to be 3.14
            Ok(v)
        }

        forward_to_deserialize_any! {
            u64, i64, string, bytes, seq, map, struct, unit, unit_struct, newtype_struct,
            tuple, tuple_struct, option, identifier, ...
        }
    }

    let parser_number = ParserNumber::F64(3.14);
    let result = parser_number.visit(TestVisitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 3.14);
}

#[test]
fn test_visit_i64() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = i64;

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> {
            assert_eq!(v, -7); // Expecting value to be -7
            Ok(v)
        }

        forward_to_deserialize_any! {
            u64, f64, string, bytes, seq, map, struct, unit, unit_struct, newtype_struct,
            tuple, tuple_struct, option, identifier, ...
        }
    }

    let parser_number = ParserNumber::I64(-7);
    let result = parser_number.visit(TestVisitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), -7);
}

#[cfg(feature = "arbitrary_precision")]
#[test]
fn test_visit_string() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = String;

        fn visit_map<V>(self, _map: V) -> Result<Self::Value, V::Error>
        where
            V: de::MapAccess<'de>,
        {
            Ok("arbitrary precision".to_string())
        }

        forward_to_deserialize_any! {
            u64, i64, f64, string, bytes, seq, struct, unit, unit_struct, newtype_struct,
            tuple, tuple_struct, option, identifier, ...
        }
    }

    let parser_number = ParserNumber::String("12345678901234567890".to_string());
    let result = parser_number.visit(TestVisitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "arbitrary precision");
}

