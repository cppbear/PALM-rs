// Answer 0

#[test]
fn test_error_fmt() {
    use std::fmt;

    struct Error {
        kind: std::io::ErrorKind,
        message: String,
    }

    impl Error {
        fn get_ref(&self) -> &String {
            &self.message
        }
    }

    impl fmt::Debug for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_tuple("http::Error")
                .field(self.get_ref())
                .finish()
        }
    }

    let error = Error {
        kind: std::io::ErrorKind::NotFound,
        message: String::from("Resource not found"),
    };

    let mut output = String::new();
    {
        let mut formatter = fmt::Formatter::new(&mut output);
        error.fmt(&mut formatter).unwrap();
    }

    assert_eq!(output, "http::Error(\"Resource not found\")\n");
}

