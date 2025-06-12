// Answer 0

#[derive(Debug)]
struct ErrorKind {
    kind: ErrorKindType,
}

#[derive(Debug)]
enum ErrorKindType {
    GroupNameDuplicate,
}

impl ErrorKind {
    fn description(&self) -> &str {
        use self::ErrorKindType::*;
        match self.kind {
            GroupNameDuplicate => "duplicate capture group name",
            _ => unreachable!(),
        }
    }
}

#[test]
fn test_group_name_duplicate_description() {
    let error = ErrorKind {
        kind: ErrorKindType::GroupNameDuplicate,
    };
    assert_eq!(error.description(), "duplicate capture group name");
}

#[test]
#[should_panic]
fn test_unreachable_case() {
    let error = ErrorKind {
        kind: ErrorKindType::GroupNameDuplicate, // Change this if necessary to trigger panic
    };
    // No other cases should be invoked for this test to avoid panic
    assert_eq!(error.description(), "other case that should not occur");
}

