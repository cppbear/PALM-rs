// Answer 0

#[test]
fn test_visit_str_valid_single_char() {
    struct TestVisitor;
    
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = char;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a single character")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            let mut iter = v.chars();
            match (iter.next(), iter.next()) {
                (Some(c), None) => Ok(c),
                _ => Err(Error::invalid_value(Unexpected::Str(v), &self)),
            }
        }
    }

    let visitor = TestVisitor;
    let result = visitor.visit_str("a");
    assert_eq!(result.unwrap(), 'a');
}

#[test]
fn test_visit_str_invalid_multiple_chars() {
    struct TestVisitor;
    
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = char;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a single character")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            let mut iter = v.chars();
            match (iter.next(), iter.next()) {
                (Some(c), None) => Ok(c),
                _ => Err(Error::invalid_value(Unexpected::Str(v), &self)),
            }
        }
    }

    let visitor = TestVisitor;
    let result = visitor.visit_str("ab");
    assert!(result.is_err());
}

#[test]
fn test_visit_str_invalid_empty_string() {
    struct TestVisitor;
    
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = char;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a single character")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            let mut iter = v.chars();
            match (iter.next(), iter.next()) {
                (Some(c), None) => Ok(c),
                _ => Err(Error::invalid_value(Unexpected::Str(v), &self)),
            }
        }
    }

    let visitor = TestVisitor;
    let result = visitor.visit_str("");
    assert!(result.is_err());
}

