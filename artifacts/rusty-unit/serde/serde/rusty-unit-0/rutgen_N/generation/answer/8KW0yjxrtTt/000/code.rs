// Answer 0

#[test]
fn test_visit_bytes_valid_utf8() {
    struct Visitor;

    impl serde::de::Visitor for Visitor {
        type Value = String;

        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            std::str::from_utf8(v)
                .map(From::from)
                .map_err(|_| E::invalid_value(serde::de::Unexpected::Bytes(v), &self))
        }
    }

    let visitor = Visitor;
    let input = b"valid utf8";
    let result = visitor.visit_bytes(input);
    assert_eq!(result.unwrap(), "valid utf8");
}

#[test]
#[should_panic]
fn test_visit_bytes_invalid_utf8() {
    struct Visitor;

    impl serde::de::Visitor for Visitor {
        type Value = String;

        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            std::str::from_utf8(v)
                .map(From::from)
                .map_err(|_| E::invalid_value(serde::de::Unexpected::Bytes(v), &self))
        }
    }

    let visitor = Visitor;
    let input = b"\xFF\xFE\xFD"; // Invalid UTF-8
    let _result = visitor.visit_bytes(input).unwrap(); // This should panic
}

