// Answer 0

#[derive(Debug)]
struct Span {
    start: usize,
    end: usize,
}

#[derive(Debug)]
enum ErrorKind {
    FlagDuplicate { original: Span },
    FlagRepeatedNegation { original: Span, additional_info: String },
    GroupNameDuplicate { original: Span, additional_info: String },
    OtherError,
}

struct MyError {
    kind: ErrorKind,
}

impl MyError {
    pub fn auxiliary_span(&self) -> Option<&Span> {
        use ErrorKind::*;
        match self.kind {
            FlagDuplicate { ref original } => Some(original),
            FlagRepeatedNegation { ref original, .. } => Some(original),
            GroupNameDuplicate { ref original, .. } => Some(original),
            _ => None,
        }
    }
}

#[test]
fn test_auxiliary_span_other_error() {
    let error = MyError {
        kind: ErrorKind::OtherError,
    };
    assert_eq!(error.auxiliary_span(), None);
}

#[test]
fn test_auxiliary_span_flag_duplicate() {
    let error = MyError {
        kind: ErrorKind::FlagDuplicate { original: Span { start: 0, end: 1 }},
    };
    assert_eq!(error.auxiliary_span(), Some(&Span { start: 0, end: 1 }));
} 

#[test]
fn test_auxiliary_span_flag_repeated_negation() {
    let error = MyError {
        kind: ErrorKind::FlagRepeatedNegation { original: Span { start: 2, end: 3 }, additional_info: String::from("info") },
    };
    assert_eq!(error.auxiliary_span(), Some(&Span { start: 2, end: 3 }));
}

#[test]
fn test_auxiliary_span_group_name_duplicate() {
    let error = MyError {
        kind: ErrorKind::GroupNameDuplicate { original: Span { start: 4, end: 5 }, additional_info: String::from("info") },
    };
    assert_eq!(error.auxiliary_span(), Some(&Span { start: 4, end: 5 }));
}

