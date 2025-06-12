// Answer 0

#[test]
fn test_fmt_str_variant() {
    use std::fmt;

    struct Unexpected {
        data: String,
    }

    impl Unexpected {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Unexpected { data } => write!(formatter, "string {:?}", data),
            }
        }
    }

    let instance = Unexpected {
        data: String::from("test string"),
    };

    let mut output = String::new();
    let result = instance.fmt(&mut fmt::Formatter::new(&mut output));
    
    assert!(result.is_ok());
    assert_eq!(output, "string \"test string\"");
}

#[test]
fn test_fmt_str_variant_empty() {
    use std::fmt;

    struct Unexpected {
        data: String,
    }

    impl Unexpected {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Unexpected { data } => write!(formatter, "string {:?}", data),
            }
        }
    }

    let instance = Unexpected {
        data: String::from(""),
    };

    let mut output = String::new();
    let result = instance.fmt(&mut fmt::Formatter::new(&mut output));
    
    assert!(result.is_ok());
    assert_eq!(output, "string \"\"");
}

