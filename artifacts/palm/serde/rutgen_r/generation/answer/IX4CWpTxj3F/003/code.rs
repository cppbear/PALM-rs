// Answer 0

#[test]
fn test_fmt_tuple_variant() {
    use std::fmt;

    struct Unexpected {
        kind: UnexpectedKind,
    }

    enum UnexpectedKind {
        TupleVariant,
    }

    impl fmt::Display for Unexpected {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match self.kind {
                UnexpectedKind::TupleVariant => formatter.write_str("tuple variant"),
            }
        }
    }

    let variant = Unexpected { kind: UnexpectedKind::TupleVariant };
    let mut buffer = String::new();
    let result = variant.fmt(&mut fmt::Formatter::new(&mut buffer));
    assert!(result.is_ok());
    assert_eq!(buffer, "tuple variant");
}

