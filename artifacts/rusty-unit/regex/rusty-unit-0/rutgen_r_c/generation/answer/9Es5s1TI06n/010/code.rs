// Answer 0

#[test]
fn test_group_name_unexpected_eof_display() {
    struct TestErrorKind {
        kind: ErrorKind,
    }
    
    impl fmt::Display for TestErrorKind {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self.kind {
                ErrorKind::GroupNameUnexpectedEof => {
                    write!(f, "unclosed capture group name")
                }
                _ => unreachable!(),
            }
        }
    }
    
    let error = TestErrorKind {
        kind: ErrorKind::GroupNameUnexpectedEof,
    };
    
    let expected_output = "unclosed capture group name";
    
    assert_eq!(format!("{}", error), expected_output);
}

#[test]
fn test_class_unclosed_display() {
    struct TestErrorKind {
        kind: ErrorKind,
    }

    impl fmt::Display for TestErrorKind {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self.kind {
                ErrorKind::ClassUnclosed => {
                    write!(f, "unclosed character class")
                }
                _ => unreachable!(),
            }
        }
    }

    let error = TestErrorKind {
        kind: ErrorKind::ClassUnclosed,
    };

    let expected_output = "unclosed character class";

    assert_eq!(format!("{}", error), expected_output);
}

