// Answer 0

#[test]
fn test_description_group_name_invalid() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct TestError {
        kind: ErrorKind,
    }

    impl error::Error for TestError {
        fn description(&self) -> &str {
            use self::ErrorKind::*;
            match self.kind {
                GroupNameInvalid => "invalid capture group name",
                _ => unreachable!(),
            }
        }
    }

    let error = TestError {
        kind: ErrorKind::GroupNameInvalid,
    };

    assert_eq!(error.description(), "invalid capture group name");
}

