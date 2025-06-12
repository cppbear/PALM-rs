// Answer 0

#[test]
fn test_unexpected_newtype_variant_display() {
    struct WithDecimalPoint(f64); // Required for formatting float

    #[derive(Copy, Clone, PartialEq, Debug)]
    pub enum Unexpected<'a> {
        NewtypeVariant,
        Other(&'a str),
    }

    impl<'a> fmt::Display for Unexpected<'a> {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Unexpected::NewtypeVariant => formatter.write_str("newtype variant"),
                Unexpected::Other(other) => formatter.write_str(other),
            }
        }
    }

    let unexpected_variant = Unexpected::NewtypeVariant;
    let mut output = String::new();
    let result = unexpected_variant.fmt(&mut output).unwrap();

    assert_eq!(output, "newtype variant");
}

