// Answer 0

#[test]
fn test_expecting_success() {
    struct TestVisitor;

    impl<'a> Visitor<'a> for TestVisitor {
        type Value = &'a Path;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a borrowed path")
        }

        fn visit_borrowed_str<E>(self, _: &'a str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(Path::new("/test/path"))
        }

        fn visit_borrowed_bytes<E>(self, v: &'a [u8]) -> Result<Self::Value, E>
        where
            E: Error,
        {
            std::str::from_utf8(v)
                .map(AsRef::as_ref)
                .map_err(|_| Error::invalid_value(Unexpected::Bytes(v), &self))
        }
    }

    let mut buf = String::new();
    let formatter = &mut fmt::Formatter::new(&mut buf);
    let visitor = TestVisitor;
    let result = visitor.expecting(formatter);

    assert!(result.is_ok());
    assert_eq!(buf, "a borrowed path");
}

#[test]
#[should_panic]
fn test_expecting_failure_on_visit_borrowed_str() {
    struct PanickingVisitor;

    impl<'a> Visitor<'a> for PanickingVisitor {
        type Value = &'a Path;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a borrowed path")
        }

        fn visit_borrowed_str<E>(self, _: &'a str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            panic!("This should panic when invoked");
        }
    }

    let mut buf = String::new();
    let formatter = &mut fmt::Formatter::new(&mut buf);
    let visitor = PanickingVisitor;

    // Here we check that the visitor panics when invoking the method
    let _ = visitor.visit_borrowed_str("test");
} 

#[test]
#[should_panic]
fn test_expecting_failure_on_visit_borrowed_bytes() {
    struct PanickingVisitor;

    impl<'a> Visitor<'a> for PanickingVisitor {
        type Value = &'a Path;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a borrowed path")
        }

        fn visit_borrowed_bytes<E>(self, _: &'a [u8]) -> Result<Self::Value, E>
        where
            E: Error,
        {
            panic!("This should panic when invoked");
        }
    }

    let visitor = PanickingVisitor;

    // Here we check that the visitor panics when invoking the method
    let _ = visitor.visit_borrowed_bytes(b"test");
}

