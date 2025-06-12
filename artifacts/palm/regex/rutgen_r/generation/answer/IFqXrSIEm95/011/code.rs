// Answer 0

#[derive(Debug)]
struct ErrorKind {
    kind: ErrorType,
}

#[derive(Debug)]
enum ErrorType {
    GroupNameInvalid,
}

impl ErrorKind {
    fn description(&self) -> &str {
        use ErrorType::*;
        match self.kind {
            GroupNameInvalid => "invalid capture group name",
            _ => unreachable!(),
        }
    }
}

#[test]
fn test_description_group_name_invalid() {
    let error = ErrorKind {
        kind: ErrorType::GroupNameInvalid,
    };
    assert_eq!(error.description(), "invalid capture group name");
}

