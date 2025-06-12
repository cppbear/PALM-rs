// Answer 0

#[test]
fn test_expectation_for_char_visitor() {
    struct LocalCharVisitor;

    impl<'de> Visitor<'de> for LocalCharVisitor {
        type Value = char;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a character")
        }
        
        fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(v)
        }
    }

    let mut output = String::new();
    let mut formatter = fmt::Formatter::with_str(&mut output);
    let visitor = LocalCharVisitor;

    let result = visitor.expecting(&mut formatter);
    assert!(result.is_ok());
    assert_eq!(output, "a character");
}

#[test]
fn test_visit_char_success() {
    struct LocalCharVisitor;

    impl<'de> Visitor<'de> for LocalCharVisitor {
        type Value = char;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a character")
        }
        
        fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(v)
        }
    }

    let visitor = LocalCharVisitor;
    let result = visitor.visit_char('a');
    assert_eq!(result, Ok('a'));
}

#[test]
#[should_panic(expected = "invalid value: string")]
fn test_visit_str_failure() {
    struct LocalCharVisitor;

    impl<'de> Visitor<'de> for LocalCharVisitor {
        type Value = char;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a character")
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

    let visitor = LocalCharVisitor;
    let _ = visitor.visit_str("not a char");
}

#[test]
fn test_visit_str_success() {
    struct LocalCharVisitor;

    impl<'de> Visitor<'de> for LocalCharVisitor {
        type Value = char;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a character")
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

    let visitor = LocalCharVisitor;
    let result = visitor.visit_str("a");
    assert_eq!(result, Ok('a'));
}

