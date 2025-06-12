// Answer 0

#[test]
fn test_fmt_group_name_unexpected_eof() {
    use std::fmt;

    #[derive(Debug)]
    enum ErrorKind {
        GroupNameUnexpectedEof,
        // other variants omitted for brevity
    }

    struct Error(ErrorKind);

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self.0 {
                ErrorKind::GroupNameUnexpectedEof => {
                    write!(f, "unclosed capture group name")
                }
                // other match arms omitted for brevity
            }
        }
    }

    let error = Error(ErrorKind::GroupNameUnexpectedEof);
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    
    assert!(result.is_ok());
    assert_eq!(output, "unclosed capture group name");
}

