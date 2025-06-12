// Answer 0

#[test]
fn test_description_group_unclosed() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Error {
        kind: ErrorKind,
    }

    #[derive(Clone, Copy, Eq, PartialEq)]
    enum ErrorKind {
        GroupUnclosed,
    }

    let error = Error {
        kind: ErrorKind::GroupUnclosed,
    };

    assert_eq!(error.description(), "unclosed group");
}

