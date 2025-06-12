// Answer 0

#[derive(Debug)]
struct Span {
    start: usize,
    end: usize,
}

#[derive(Debug)]
enum ErrorKind {
    FlagDuplicate { original: Span },
    FlagRepeatedNegation { original: Span, extra: usize },
    GroupNameDuplicate { original: Span, extra: usize },
    Other,
}

struct Error {
    kind: ErrorKind,
}

impl Error {
    pub fn auxiliary_span(&self) -> Option<&Span> {
        use self::ErrorKind::*;
        match self.kind {
            FlagDuplicate { ref original } => Some(original),
            FlagRepeatedNegation { ref original, .. } => Some(original),
            GroupNameDuplicate { ref original, .. } => Some(original),
            _ => None,
        }
    }
}

#[test]
fn test_auxiliary_span_flag_duplicate() {
    let original_span = Span { start: 5, end: 10 };
    let error = Error {
        kind: ErrorKind::FlagDuplicate { original: original_span },
    };
    assert_eq!(error.auxiliary_span(), Some(&Span { start: 5, end: 10 }));
}

#[test]
fn test_auxiliary_span_flag_repeated_negation() {
    let original_span = Span { start: 3, end: 8 };
    let error = Error {
        kind: ErrorKind::FlagRepeatedNegation { original: original_span, extra: 2 },
    };
    assert_eq!(error.auxiliary_span(), Some(&Span { start: 3, end: 8 }));
}

#[test]
fn test_auxiliary_span_group_name_duplicate() {
    let original_span = Span { start: 0, end: 4 };
    let error = Error {
        kind: ErrorKind::GroupNameDuplicate { original: original_span, extra: 1 },
    };
    assert_eq!(error.auxiliary_span(), Some(&Span { start: 0, end: 4 }));
}

#[test]
fn test_auxiliary_span_other() {
    let error = Error {
        kind: ErrorKind::Other,
    };
    assert_eq!(error.auxiliary_span(), None);
}

