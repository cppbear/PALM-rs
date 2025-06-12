// Answer 0

#[test]
fn test_description_class_range_invalid() {
    struct ErrorKind {
        kind: Kind,
    }

    enum Kind {
        ClassRangeInvalid,
    }

    impl ErrorKind {
        fn description(&self) -> &str {
            use self::Kind::*;
            match self.kind {
                ClassRangeInvalid => "invalid character class range",
            }
        }
    }

    let error = ErrorKind { kind: Kind::ClassRangeInvalid };
    assert_eq!(error.description(), "invalid character class range");
}

#[test]
fn test_description_class_range_invalid_multiple_conditions() {
    struct ErrorKind {
        kind: Kind,
    }

    enum Kind {
        ClassRangeInvalid,
        ClassRangeInvalid2,
    }

    impl ErrorKind {
        fn description(&self) -> &str {
            use self::Kind::*;
            match self.kind {
                ClassRangeInvalid | ClassRangeInvalid2 => "invalid character class range",
            }
        }
    }

    let error1 = ErrorKind { kind: Kind::ClassRangeInvalid };
    let error2 = ErrorKind { kind: Kind::ClassRangeInvalid2 };
    assert_eq!(error1.description(), "invalid character class range");
    assert_eq!(error2.description(), "invalid character class range");
}

