// Answer 0

#[test]
fn test_visit_str_valid_integer() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = i32;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a valid integer")
        }

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            s.parse().map_err(Error::custom)
        }
    }

    let visitor = TestVisitor;
    let result: Result<i32, _> = visitor.visit_str("42");
    assert_eq!(result, Ok(42));
}

#[test]
fn test_visit_str_invalid_integer() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = i32;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a valid integer")
        }

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            s.parse().map_err(Error::custom)
        }
    }

    let visitor = TestVisitor;
    let result: Result<i32, _> = visitor.visit_str("not_a_number");
    assert!(result.is_err());
}

#[test]
fn test_visit_str_empty_string() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = i32;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a valid integer")
        }

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            s.parse().map_err(Error::custom)
        }
    }

    let visitor = TestVisitor;
    let result: Result<i32, _> = visitor.visit_str("");
    assert!(result.is_err());
}

#[test]
fn test_visit_str_negative_integer() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = i32;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a valid integer")
        }

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            s.parse().map_err(Error::custom)
        }
    }

    let visitor = TestVisitor;
    let result: Result<i32, _> = visitor.visit_str("-10");
    assert_eq!(result, Ok(-10));
}

#[test]
fn test_visit_str_large_integer() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = i32;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a valid integer")
        }

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            s.parse().map_err(Error::custom)
        }
    }

    let visitor = TestVisitor;
    let result: Result<i32, _> = visitor.visit_str("2147483648"); // This exceeds i32 max
    assert!(result.is_err());
}

