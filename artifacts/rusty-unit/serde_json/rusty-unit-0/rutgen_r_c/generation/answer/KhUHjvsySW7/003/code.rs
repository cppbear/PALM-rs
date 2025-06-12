// Answer 0

fn test_visit_f64() {
    struct TestVisitor {
        value: Option<f64>,
    }

    impl de::Visitor<'_> for TestVisitor {
        type Value = f64;

        fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E> {
            Ok(v)
        }

        forward_to_deserialize_any! {
            bool i8 i16 i32 i64 u8 u16 u32 u64 f32 char str string unit seq map option enum identifier bytes byte_buf
        }
    }

    let parser_number = ParserNumber::F64(3.14);
    let visitor = TestVisitor { value: None };
    let result = parser_number.visit(visitor);
    assert_eq!(result.unwrap(), 3.14);
}

fn test_visit_u64() {
    struct TestVisitor {
        value: Option<u64>,
    }

    impl de::Visitor<'_> for TestVisitor {
        type Value = u64;

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
            Ok(v)
        }

        forward_to_deserialize_any! {
            bool i8 i16 i32 i64 f64 u8 u16 u32 string unit seq map option enum identifier bytes byte_buf
        }
    }

    let parser_number = ParserNumber::U64(42);
    let visitor = TestVisitor { value: None };
    let result = parser_number.visit(visitor);
    assert_eq!(result.unwrap(), 42);
}

fn test_visit_i64() {
    struct TestVisitor {
        value: Option<i64>,
    }

    impl de::Visitor<'_> for TestVisitor {
        type Value = i64;

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> {
            Ok(v)
        }

        forward_to_deserialize_any! {
            bool u8 u16 u32 u64 f64 i8 i16 string unit seq map option enum identifier bytes byte_buf
        }
    }

    let parser_number = ParserNumber::I64(-100);
    let visitor = TestVisitor { value: None };
    let result = parser_number.visit(visitor);
    assert_eq!(result.unwrap(), -100);
}

#[cfg(feature = "arbitrary_precision")]
fn test_visit_string() {
    struct TestVisitor {
        value: Option<String>,
    }

    impl de::Visitor<'_> for TestVisitor {
        type Value = String;

        fn visit_map<V>(self, _: V) -> Result<Self::Value, V::Error>
        where
            V: de::MapAccess<'_>,
        {
            Ok("arbitrary_precision_string".to_string())
        }

        forward_to_deserialize_any! {
            bool i8 i16 i32 i64 u8 u16 u32 u64 f64 char str string unit seq option enum identifier bytes byte_buf
        }
    }

    let parser_number = ParserNumber::String("test".to_string());
    let visitor = TestVisitor { value: None };
    let result = parser_number.visit(visitor);
    assert_eq!(result.unwrap(), "arbitrary_precision_string".to_string());
}

fn main() {
    test_visit_f64();
    test_visit_u64();
    test_visit_i64();
    #[cfg(feature = "arbitrary_precision")]
    test_visit_string();
}

