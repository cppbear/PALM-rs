// Answer 0

#[test]
fn test_visit_string_success() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = CString;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a valid C string")
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: Error,
        {
            CString::new(v).map_err(Error::custom)
        }
    }

    let visitor = TestVisitor;
    let valid_string = String::from("Hello, world!");
    let result: Result<CString, _> = visitor.visit_string(valid_string);
    assert!(result.is_ok());
}

#[test]
#[should_panic(expected = "NUL byte found in string")]
fn test_visit_string_failure_with_null_byte() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = CString;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a valid C string")
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: Error,
        {
            CString::new(v).map_err(Error::custom)
        }
    }

    let visitor = TestVisitor;
    let invalid_string = String::from("Hello\0world!");
    let _result: Result<CString, _> = visitor.visit_string(invalid_string);
} 

#[test]
#[should_panic(expected = "invalid UTF-8")]
fn test_visit_string_failure_with_invalid_utf8() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = CString;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a valid C string")
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: Error,
        {
            CString::new(v).map_err(Error::custom)
        }
    }

    let visitor = TestVisitor;
    let invalid_utf8_bytes = vec![0xFF, 0xFF, 0xFF];
    let invalid_utf8_string = String::from_utf8(invalid_utf8_bytes).unwrap();
    let _result: Result<CString, _> = visitor.visit_string(invalid_utf8_string);
}

