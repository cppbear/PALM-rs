// Answer 0

#[derive(Debug)]
struct Span {
    start: usize,
    end: usize,
}

#[derive(Debug)]
enum ErrorKind {
    FlagDuplicate { original: Span },
    FlagRepeatedNegation { original: Span, another: usize },
    GroupNameDuplicate { original: Span, name: String },
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
fn test_auxiliary_span_group_name_duplicate() {
    let original_span = Span { start: 5, end: 10 };
    let error = Error {
        kind: ErrorKind::GroupNameDuplicate {
            original: original_span,
            name: String::from("duplicate_group"),
        },
    };
    assert_eq!(error.auxiliary_span(), Some(&original_span));
}

