// Answer 0

#[test]
fn test_group_unopened_display() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct TestError {
        kind: ErrorKind,
    }

    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.kind.fmt(f)
        }
    }

    let error = TestError {
        kind: ErrorKind::GroupUnopened,
    };
    
    assert_eq!(format!("{}", error), "unopened group");
}

#[test]
fn test_group_name_empty_display() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct TestError {
        kind: ErrorKind,
    }

    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.kind.fmt(f)
        }
    }

    let error = TestError {
        kind: ErrorKind::GroupNameEmpty,
    };
    
    assert_eq!(format!("{}", error), "empty capture group name");
}

