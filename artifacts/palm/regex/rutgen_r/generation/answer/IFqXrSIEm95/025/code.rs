// Answer 0

#[derive(Debug)]
struct ErrorKind {
    kind: Kind,
}

#[derive(Debug)]
enum Kind {
    DecimalEmpty,
}

impl ErrorKind {
    fn description(&self) -> &str {
        match self.kind {
            Kind::DecimalEmpty => "empty decimal literal",
        }
    }
}

#[test]
fn test_description_decimal_empty() {
    let error = ErrorKind { kind: Kind::DecimalEmpty };
    assert_eq!(error.description(), "empty decimal literal");
}

