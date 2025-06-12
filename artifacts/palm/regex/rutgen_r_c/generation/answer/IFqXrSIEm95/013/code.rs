// Answer 0

#[test]
fn test_description_group_name_duplicate() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Error {
        kind: ErrorKind,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    enum ErrorKind {
        GroupNameDuplicate { original: Span },
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Span {
        start: usize,
        end: usize,
    }

    let error = Error {
        kind: ErrorKind::GroupNameDuplicate {
            original: Span { start: 0, end: 5 },
        },
    };

    assert_eq!(error.description(), "duplicate capture group name");
}

