// Answer 0

#[test]
fn test_visit_str_valid() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(v.to_owned())
        }
    }

    let visitor = TestVisitor;
    let result = visitor.visit_str("test string").unwrap();
    assert_eq!(result, "test string");
}

#[test]
#[should_panic]
fn test_visit_str_invalid_utf8() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: Error,
        {
            match std::str::from_utf8(v) {
                Ok(s) => Ok(s.to_owned()),
                Err(_) => panic!("Invalid UTF-8 sequence"),
            }
        }
    }

    let visitor = TestVisitor;
    visitor.visit_bytes(&[0, 159, 146, 150]); // invalid utf-8 bytes
}

