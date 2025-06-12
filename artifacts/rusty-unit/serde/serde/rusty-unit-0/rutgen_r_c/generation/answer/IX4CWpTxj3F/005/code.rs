// Answer 0

#[test]
fn test_fmt_unit_variant() {
    use std::fmt;

    struct WithDecimalPoint(f64);

    #[derive(Copy, Clone)]
    pub enum Unexpected<'a> {
        UnitVariant,
        // Other variants are omitted for brevity
    }

    impl<'a> fmt::Display for Unexpected<'a> {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            use self::Unexpected::*;
            match *self {
                UnitVariant => formatter.write_str("unit variant"),
                // Other variants are omitted for brevity
            }
        }
    }

    let unit_variant = Unexpected::UnitVariant;
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", unit_variant);
    
    assert!(result.is_ok());
    assert_eq!(buffer, "unit variant");
}

#[test]
fn test_fmt_bytes_variant() {
    use std::fmt;

    struct WithDecimalPoint(f64);

    #[derive(Copy, Clone)]
    pub enum Unexpected<'a> {
        Bytes(&'a [u8]),
        // Other variants are omitted for brevity
    }

    impl<'a> fmt::Display for Unexpected<'a> {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            use self::Unexpected::*;
            match *self {
                Bytes(_) => formatter.write_str("byte array"),
                // Other variants are omitted for brevity
            }
        }
    }

    let bytes_variant = Unexpected::Bytes(&[1, 2, 3]);
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", bytes_variant);
    
    assert!(result.is_ok());
    assert_eq!(buffer, "byte array");
}

