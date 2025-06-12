// Answer 0

#[test]
fn test_visit_str_valid_integer() {
    struct DummyVisitor;

    impl serde::de::Visitor for DummyVisitor {
        type Value = i32;

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            s.parse().map_err(E::custom)
        }
    }

    let visitor = DummyVisitor;
    let result: Result<i32, _> = visitor.visit_str("123");
    assert_eq!(result, Ok(123));
}

#[test]
fn test_visit_str_valid_float() {
    struct DummyVisitor;

    impl serde::de::Visitor for DummyVisitor {
        type Value = f64;

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            s.parse().map_err(E::custom)
        }
    }

    let visitor = DummyVisitor;
    let result: Result<f64, _> = visitor.visit_str("123.45");
    assert_eq!(result, Ok(123.45));
}

#[test]
fn test_visit_str_invalid_integer() {
    struct DummyVisitor;

    impl serde::de::Visitor for DummyVisitor {
        type Value = i32;

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            s.parse().map_err(E::custom)
        }
    }

    let visitor = DummyVisitor;
    let result: Result<i32, _> = visitor.visit_str("abc");
    assert!(result.is_err());
}

#[test]
fn test_visit_str_empty_string() {
    struct DummyVisitor;

    impl serde::de::Visitor for DummyVisitor {
        type Value = i32;

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            s.parse().map_err(E::custom)
        }
    }

    let visitor = DummyVisitor;
    let result: Result<i32, _> = visitor.visit_str("");
    assert!(result.is_err());
}

#[test]
fn test_visit_str_whitespace() {
    struct DummyVisitor;

    impl serde::de::Visitor for DummyVisitor {
        type Value = i32;

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            s.parse().map_err(E::custom)
        }
    }

    let visitor = DummyVisitor;
    let result: Result<i32, _> = visitor.visit_str("   ");
    assert!(result.is_err());
}

