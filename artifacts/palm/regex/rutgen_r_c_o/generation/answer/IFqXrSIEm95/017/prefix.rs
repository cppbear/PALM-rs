// Answer 0

#[test]
fn test_description_flag_duplicate_case_1() {
    let original = Span { start: Position(10), end: Position(20) };
    let error = Error {
        kind: ErrorKind::FlagDuplicate { original },
    };
    let _ = error.description();
}

#[test]
fn test_description_flag_duplicate_case_2() {
    let original = Span { start: Position(0), end: Position(1) };
    let error = Error {
        kind: ErrorKind::FlagDuplicate { original },
    };
    let _ = error.description();
}

#[test]
fn test_description_flag_duplicate_case_3() {
    let original = Span { start: Position(50), end: Position(75) };
    let error = Error {
        kind: ErrorKind::FlagDuplicate { original },
    };
    let _ = error.description();
}

#[test]
fn test_description_flag_duplicate_case_4() {
    let original = Span { start: Position(100), end: Position(100) };
    let error = Error {
        kind: ErrorKind::FlagDuplicate { original },
    };
    let _ = error.description();
}

