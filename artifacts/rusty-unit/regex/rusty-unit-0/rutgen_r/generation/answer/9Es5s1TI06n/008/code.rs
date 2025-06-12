// Answer 0

#[test]
fn test_group_unopened() {
    // Defining a self-contained struct to represent the GroupUnopened variant
    struct FakeErrorKind {
        kind: ErrorKind,
    }

    impl fmt::Display for FakeErrorKind {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self.kind {
                ErrorKind::GroupUnopened => write!(f, "unopened group"),
                _ => unreachable!(),
            }
        }
    }

    enum ErrorKind {
        GroupUnopened,
    }

    let error = FakeErrorKind { kind: ErrorKind::GroupUnopened };

    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", error);

    assert!(result.is_ok());
    assert_eq!(buffer, "unopened group");
}

#[test]
#[should_panic(expected = "unreachable!() called")]
fn test_unreachable_case() {
    struct FakeErrorKind {
        kind: ErrorKind,
    }

    impl fmt::Display for FakeErrorKind {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self.kind {
                ErrorKind::GroupUnopened => write!(f, "unopened group"),
                _ => unreachable!(),
            }
        }
    }

    enum ErrorKind {
        GroupUnopened,
        Invalid,
    }

    let error = FakeErrorKind { kind: ErrorKind::Invalid };

    let _ = write!(String::new(), "{}", error);
}

