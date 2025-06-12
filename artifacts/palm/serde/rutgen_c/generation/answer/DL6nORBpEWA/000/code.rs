// Answer 0

#[test]
fn test_bytes_visitor_expecting() {
    struct TestBytesVisitor;

    impl<'a> Visitor<'a> for TestBytesVisitor {
        type Value = &'a [u8];

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a borrowed byte array")
        }
        
        fn visit_borrowed_bytes<E>(self, v: &'a [u8]) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(v)
        }
        
        fn visit_borrowed_str<E>(self, v: &'a str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(v.as_bytes())
        }
    }

    let mut output = String::new();
    let mut formatter = fmt::Formatter::new(&mut output);
    let visitor = TestBytesVisitor;

    let result = visitor.expecting(&mut formatter);

    assert!(result.is_ok());
    assert_eq!(output, "a borrowed byte array");
}

