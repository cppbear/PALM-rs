// Answer 0

#[test]
fn test_flag_duplicate_case_1() {
    struct Span { start: u32, end: u32 }
    let original = Span { start: 5, end: 10 };
    let duplicate = Span { start: 11, end: 15 };
    let error_kind = ErrorKind::FlagDuplicate { original };

    let _ = fmt(&error_kind);
}

#[test]
fn test_flag_duplicate_case_2() {
    struct Span { start: u32, end: u32 }
    let original = Span { start: 1, end: 2 };
    let duplicate = Span { start: 3, end: 4 };
    let error_kind = ErrorKind::FlagDuplicate { original };

    let _ = fmt(&error_kind);
}

#[test]
fn test_flag_duplicate_case_3() {
    struct Span { start: u32, end: u32 }
    let original = Span { start: 15, end: 20 };
    let duplicate = Span { start: 21, end: 25 };
    let error_kind = ErrorKind::FlagDuplicate { original };

    let _ = fmt(&error_kind);
}

