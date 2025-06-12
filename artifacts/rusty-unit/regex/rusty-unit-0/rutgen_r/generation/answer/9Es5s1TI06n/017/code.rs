// Answer 0

#[test]
fn test_flag_duplicate() {
    use std::fmt;

    struct ErrorKind {
        kind: FlagDuplicate,
    }

    struct FlagDuplicate {
        flags: String,
    }

    impl fmt::Display for ErrorKind {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self.kind {
                FlagDuplicate { ref flags } => {
                    write!(f, "duplicate flag: {}", flags)
                }
            }
        }
    }

    let error = ErrorKind {
        kind: FlagDuplicate {
            flags: String::from("g"),
        },
    };

    let mut output = String::new();
    write!(&mut output, "{}", error).unwrap();

    assert_eq!(output, "duplicate flag: g");
}

#[test]
fn test_another_flag_duplicate() {
    use std::fmt;

    struct ErrorKind {
        kind: FlagDuplicate,
    }

    struct FlagDuplicate {
        flags: String,
    }

    impl fmt::Display for ErrorKind {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self.kind {
                FlagDuplicate { ref flags } => {
                    write!(f, "duplicate flag: {}", flags)
                }
            }
        }
    }

    let error = ErrorKind {
        kind: FlagDuplicate {
            flags: String::from("i"),
        },
    };

    let mut output = String::new();
    write!(&mut output, "{}", error).unwrap();

    assert_eq!(output, "duplicate flag: i");
}

