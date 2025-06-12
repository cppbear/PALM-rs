// Answer 0

#[derive(Debug)]
enum ErrorKind {
    GroupNameDuplicate { name: String },
    // other variants omitted for brevity
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use self::ErrorKind::*;
        match *self {
            GroupNameDuplicate { ref name } => {
                write!(f, "duplicate capture group name: {}", name)
            }
            // other patterns omitted for brevity
        }
    }
}

#[test]
fn test_group_name_duplicate() {
    let error = ErrorKind::GroupNameDuplicate { name: "group1".to_string() };
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);

    assert!(result.is_ok());
    assert_eq!(output, "duplicate capture group name: group1");
}

#[test]
fn test_group_name_duplicate_empty() {
    let error = ErrorKind::GroupNameDuplicate { name: "".to_string() };
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);

    assert!(result.is_ok());
    assert_eq!(output, "duplicate capture group name: ");
}

