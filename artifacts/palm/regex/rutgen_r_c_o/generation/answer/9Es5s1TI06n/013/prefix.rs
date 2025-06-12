// Answer 0

#[test]
fn test_group_name_duplicate() {
    let original_span = Span { start: Position(0), end: Position(1) };
    let error_kind = ErrorKind::GroupNameDuplicate { original: original_span };
    let mut buffer = String::new();
    let _ = fmt(&error_kind, &mut buffer);
}

#[test]
fn test_group_name_duplicate_second_case() {
    let original_span = Span { start: Position(1), end: Position(2) };
    let error_kind = ErrorKind::GroupNameDuplicate { original: original_span };
    let mut buffer = String::new();
    let _ = fmt(&error_kind, &mut buffer);
}

#[test]
fn test_group_name_duplicate_with_max_span() {
    let original_span = Span { start: Position(0), end: Position(u32::MAX) };
    let error_kind = ErrorKind::GroupNameDuplicate { original: original_span };
    let mut buffer = String::new();
    let _ = fmt(&error_kind, &mut buffer);
}

