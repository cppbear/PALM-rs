// Answer 0

#[test]
fn test_visit_borrowed_str_valid() {
    struct TestVisitor;

    impl<'a> Visitor<'a> for TestVisitor {
        type Value = &'a [u8];

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a borrowed string")
        }

        fn visit_borrowed_str<E>(self, v: &'a str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(v.as_bytes())
        }
    }

    let visitor = TestVisitor;
    let test_str = "Hello, World!";
    let result = visitor.visit_borrowed_str(test_str);
    assert_eq!(result.unwrap(), test_str.as_bytes());
}

#[test]
fn test_visit_borrowed_str_empty() {
    struct TestVisitor;

    impl<'a> Visitor<'a> for TestVisitor {
        type Value = &'a [u8];

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a borrowed string")
        }

        fn visit_borrowed_str<E>(self, v: &'a str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(v.as_bytes())
        }
    }

    let visitor = TestVisitor;
    let test_str = "";
    let result = visitor.visit_borrowed_str(test_str);
    assert_eq!(result.unwrap(), test_str.as_bytes());
}

