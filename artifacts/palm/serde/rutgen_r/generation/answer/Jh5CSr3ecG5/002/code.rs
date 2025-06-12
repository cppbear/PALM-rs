// Answer 0

#[test]
fn test_visit_str_empty_string() {
    struct TestVisitor;

    impl TestVisitor {
        fn visit_str<E>(self, v: &str) -> Result<char, E>
        where
            E: serde::de::Error,
        {
            let mut iter = v.chars();
            match (iter.next(), iter.next()) {
                (Some(c), None) => Ok(c),
                _ => Err(E::invalid_value(serde::de::Unexpected::Str(v), &self)),
            }
        }
    }

    let visitor = TestVisitor;
    let result: Result<char, serde::de::Error> = visitor.visit_str("");
    assert!(result.is_err());
}

#[test]
fn test_visit_str_multiple_characters() {
    struct TestVisitor;

    impl TestVisitor {
        fn visit_str<E>(self, v: &str) -> Result<char, E>
        where
            E: serde::de::Error,
        {
            let mut iter = v.chars();
            match (iter.next(), iter.next()) {
                (Some(c), None) => Ok(c),
                _ => Err(E::invalid_value(serde::de::Unexpected::Str(v), &self)),
            }
        }
    }

    let visitor = TestVisitor;
    let result: Result<char, serde::de::Error> = visitor.visit_str("abc");
    assert!(result.is_err());
}

#[test]
fn test_visit_str_whitespace() {
    struct TestVisitor;

    impl TestVisitor {
        fn visit_str<E>(self, v: &str) -> Result<char, E>
        where
            E: serde::de::Error,
        {
            let mut iter = v.chars();
            match (iter.next(), iter.next()) {
                (Some(c), None) => Ok(c),
                _ => Err(E::invalid_value(serde::de::Unexpected::Str(v), &self)),
            }
        }
    }

    let visitor = TestVisitor;
    let result: Result<char, serde::de::Error> = visitor.visit_str(" ");
    assert!(result.is_err());
}

