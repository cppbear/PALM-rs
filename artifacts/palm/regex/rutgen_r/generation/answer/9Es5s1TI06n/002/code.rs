// Answer 0

#[test]
fn test_fmt_unsupported_look_around() {
    use std::fmt;

    #[derive(Debug)]
    enum ErrorKind {
        UnsupportedLookAround,
    }

    impl fmt::Display for ErrorKind {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                ErrorKind::UnsupportedLookAround => {
                    write!(f, "look-around, including look-ahead and look-behind, is not supported")
                }
            }
        }
    }

    let error = ErrorKind::UnsupportedLookAround;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);

    assert!(result.is_ok());
    assert_eq!(output, "look-around, including look-ahead and look-behind, is not supported");
}

