// Answer 0

#[derive(Debug)]
struct ErrorKind {
    kind: Kind,
}

#[derive(Debug)]
enum Kind {
    ClassRangeLiteral,
    // Add other variants only if needed for tests.
}

impl ErrorKind {
    fn description(&self) -> &str {
        match self.kind {
            Kind::ClassRangeLiteral => "invalid range boundary, must be a literal",
            _ => unreachable!(),
        }
    }
}

#[test]
fn test_class_range_literal_description() {
    let error = ErrorKind {
        kind: Kind::ClassRangeLiteral,
    };
    assert_eq!(error.description(), "invalid range boundary, must be a literal");
}

