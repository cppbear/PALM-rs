// Answer 0

#[test]
fn test_unexpected_str() {
    use std::fmt;

    struct WithDecimalPoint(f64);

    #[derive(Copy, Clone)]
    pub enum Unexpected<'a> {
        Str(&'a str),
    }

    impl<'a> fmt::Display for Unexpected<'a> {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            use self::Unexpected::*;
            match *self {
                Str(s) => write!(formatter, "string {:?}", s),
            }
        }
    }

    let unexpected = Unexpected::Str("test string");
    let mut output = String::new();
    let result = unexpected.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, r#"string "test string""#);
}

#[test]
fn test_unexpected_str_empty() {
    use std::fmt;

    struct WithDecimalPoint(f64);

    #[derive(Copy, Clone)]
    pub enum Unexpected<'a> {
        Str(&'a str),
    }

    impl<'a> fmt::Display for Unexpected<'a> {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            use self::Unexpected::*;
            match *self {
                Str(s) => write!(formatter, "string {:?}", s),
            }
        }
    }

    let unexpected = Unexpected::Str("");
    let mut output = String::new();
    let result = unexpected.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, r#"string ""#);
}

#[test]
fn test_unexpected_str_special_characters() {
    use std::fmt;

    struct WithDecimalPoint(f64);

    #[derive(Copy, Clone)]
    pub enum Unexpected<'a> {
        Str(&'a str),
    }

    impl<'a> fmt::Display for Unexpected<'a> {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            use self::Unexpected::*;
            match *self {
                Str(s) => write!(formatter, "string {:?}", s),
            }
        }
    }

    let unexpected = Unexpected::Str("special chars: \n\t \"'\\");
    let mut output = String::new();
    let result = unexpected.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, r#"string "special chars: \n\t \"'\\ ""#);
}

