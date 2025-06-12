// Answer 0

#[test]
fn test_fmt_enum_variant() {
    use std::fmt;

    struct Unexpected {
        kind: Kind,
    }

    enum Kind {
        Enum,
        Other(&'static str),
    }

    impl fmt::Display for Unexpected {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match self.kind {
                Kind::Enum => formatter.write_str("enum"),
                Kind::Other(ref s) => formatter.write_str(s),
            }
        }
    }

    let unexpected_enum = Unexpected { kind: Kind::Enum };
    let mut output = String::new();
    let result = write!(&mut output, "{}", unexpected_enum);
    
    assert!(result.is_ok());
    assert_eq!(output, "enum");
}

#[test]
fn test_fmt_other_variant() {
    use std::fmt;

    struct Unexpected {
        kind: Kind,
    }

    enum Kind {
        Enum,
        Other(&'static str),
    }

    impl fmt::Display for Unexpected {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match self.kind {
                Kind::Enum => formatter.write_str("enum"),
                Kind::Other(ref s) => formatter.write_str(s),
            }
        }
    }

    let unexpected_other = Unexpected { kind: Kind::Other("custom variant") };
    let mut output = String::new();
    let result = write!(&mut output, "{}", unexpected_other);
    
    assert!(result.is_ok());
    assert_eq!(output, "custom variant");
}

