// Answer 0

struct ErrorKind {
    kind: Kind,
}

enum Kind {
    RepetitionMissing,
}

impl ErrorKind {
    fn description(&self) -> &str {
        use self::Kind::*;
        match self.kind {
            RepetitionMissing => "repetition operator missing expression",
            _ => unreachable!(),
        }
    }
}

#[test]
fn test_repetition_missing_description() {
    let error = ErrorKind { kind: Kind::RepetitionMissing };
    assert_eq!(error.description(), "repetition operator missing expression");
}

