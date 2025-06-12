// Answer 0

#[derive(Debug)]
enum ErrorKind {
    GroupNameInvalid,
    // Other variants omitted for brevity
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ErrorKind::GroupNameInvalid => write!(f, "invalid capture group character"),
            // Other matches omitted for brevity
        }
    }
}

#[test]
fn test_group_name_invalid() {
    let error = ErrorKind::GroupNameInvalid;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "invalid capture group character");
}

#[test]
fn test_group_name_empty() {
    let error = ErrorKind::GroupNameEmpty;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "empty capture group name");
}

#[test]
fn test_group_name_duplicate() {
    let error = ErrorKind::GroupNameDuplicate;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "duplicate capture group name");
}

