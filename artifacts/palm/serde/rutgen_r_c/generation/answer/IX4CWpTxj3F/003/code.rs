// Answer 0

#[test]
fn test_fmt_with_tuple_variant() {
    use std::fmt;

    struct WithDecimalPoint(f64);

    #[derive(Copy, Clone, PartialEq, Debug)]
    pub enum Unexpected<'a> {
        TupleVariant,
        Other(&'a str),
    }

    impl<'a> fmt::Display for Unexpected<'a> {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Unexpected::TupleVariant => formatter.write_str("tuple variant"),
                Unexpected::Other(other) => formatter.write_str(other),
            }
        }
    }

    let tuple_variant = Unexpected::TupleVariant;
    let mut output = String::new();
    let result = write!(&mut output, "{}", tuple_variant);

    assert!(result.is_ok());
    assert_eq!(output, "tuple variant");
}

#[test]
fn test_fmt_with_other() {
    use std::fmt;

    struct WithDecimalPoint(f64);

    #[derive(Copy, Clone, PartialEq, Debug)]
    pub enum Unexpected<'a> {
        Other(&'a str),
    }

    impl<'a> fmt::Display for Unexpected<'a> {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            if let Unexpected::Other(other) = self {
                formatter.write_str(other)
            } else {
                Ok(())
            }
        }
    }

    let other_value = Unexpected::Other("unexpected type");
    let mut output = String::new();
    let result = write!(&mut output, "{}", other_value);

    assert!(result.is_ok());
    assert_eq!(output, "unexpected type");
}

