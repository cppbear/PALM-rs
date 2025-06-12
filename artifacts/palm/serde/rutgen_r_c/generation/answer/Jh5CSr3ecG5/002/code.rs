// Answer 0

#[test]
fn test_visit_str_empty() {
    let visitor = CharVisitor;
    let result: Result<char, _> = visitor.visit_str("");
    assert!(result.is_err());
    if let Err(e) = result {
        // Check if the error matches the expected pattern.
        match e {
            Error::InvalidValue(Unexpected::Str(v), _) => assert_eq!(v, ""),
            _ => panic!("Unexpected error type"),
        }
    }
}

#[test]
fn test_visit_str_multiple_characters() {
    let visitor = CharVisitor;
    let result: Result<char, _> = visitor.visit_str("abc");
    assert!(result.is_err());
    if let Err(e) = result {
        // Check if the error matches the expected pattern.
        match e {
            Error::InvalidValue(Unexpected::Str(v), _) => assert_eq!(v, "abc"),
            _ => panic!("Unexpected error type"),
        }
    }
}

#[test]
fn test_visit_str_space() {
    let visitor = CharVisitor;
    let result: Result<char, _> = visitor.visit_str(" ");
    assert!(result.is_err());
    if let Err(e) = result {
        // Check if the error matches the expected pattern.
        match e {
            Error::InvalidValue(Unexpected::Str(v), _) => assert_eq!(v, " "),
            _ => panic!("Unexpected error type"),
        }
    }
}

#[test]
fn test_visit_str_special_characters() {
    let visitor = CharVisitor;
    let result: Result<char, _> = visitor.visit_str("!");
    assert!(result.is_err());
    if let Err(e) = result {
        // Check if the error matches the expected pattern.
        match e {
            Error::InvalidValue(Unexpected::Str(v), _) => assert_eq!(v, "!"),
            _ => panic!("Unexpected error type"),
        }
    }
}

#[test]
fn test_visit_str_non_ascii() {
    let visitor = CharVisitor;
    let result: Result<char, _> = visitor.visit_str("😊");
    assert!(result.is_err());
    if let Err(e) = result {
        // Check if the error matches the expected pattern.
        match e {
            Error::InvalidValue(Unexpected::Str(v), _) => assert_eq!(v, "😊"),
            _ => panic!("Unexpected error type"),
        }
    }
}

