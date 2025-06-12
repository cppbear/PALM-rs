// Answer 0

#[test]
fn test_visit_borrowed_str_valid() {
    struct TestVisitor;

    impl<'a> Visitor<'a> for TestVisitor {
        type Value = &'a str;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a borrowed string")
        }

        fn visit_borrowed_str<E>(self, v: &'a str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(v)
        }
    }

    let visitor = TestVisitor;
    let input = "Hello, world!";
    
    let result = visitor.visit_borrowed_str(input);
    
    assert_eq!(result, Ok("Hello, world!"));
}

#[test]
fn test_visit_borrowed_str_empty() {
    struct TestVisitor;

    impl<'a> Visitor<'a> for TestVisitor {
        type Value = &'a str;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a borrowed string")
        }

        fn visit_borrowed_str<E>(self, v: &'a str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(v)
        }
    }

    let visitor = TestVisitor;
    let input = ""; // empty string input

    let result = visitor.visit_borrowed_str(input);

    assert_eq!(result, Ok("")); // should also return Ok on empty string
}

#[test]
#[should_panic]
fn test_visit_borrowed_str_invalid_utf8() {
    struct TestVisitor;

    impl<'a> Visitor<'a> for TestVisitor {
        type Value = &'a str;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a borrowed string")
        }

        fn visit_borrowed_bytes<E>(self, v: &'a [u8]) -> Result<Self::Value, E>
        where
            E: Error,
        {
            str::from_utf8(v).map_err(|_| Error::invalid_value(Unexpected::Bytes(v), &self))?
        }

        fn visit_borrowed_str<E>(self, v: &'a str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(v)
        }
    }

    let visitor = TestVisitor;
    let input: &[u8] = &[0xFF, 0xFF]; // invalid UTF-8 bytes

    // This will panic when we attempt to convert invalid bytes to str
    visitor.visit_borrowed_bytes(input);
}

#[test]
fn test_visit_borrowed_str_special_characters() {
    struct TestVisitor;

    impl<'a> Visitor<'a> for TestVisitor {
        type Value = &'a str;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a borrowed string")
        }

        fn visit_borrowed_str<E>(self, v: &'a str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(v)
        }
    }

    let visitor = TestVisitor;
    let input = "Hello, 你好!"; // special characters in the string

    let result = visitor.visit_borrowed_str(input);

    assert_eq!(result, Ok("Hello, 你好!"));
}

