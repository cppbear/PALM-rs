// Answer 0

#[derive(Debug)]
struct ErrorKind {
    kind: ErrorKindVariant,
}

#[derive(Debug)]
enum ErrorKindVariant {
    GroupUnopened,
    // other variants omitted for brevity
}

impl ErrorKind {
    fn description(&self) -> &str {
        use self::ErrorKindVariant::*;
        match self.kind {
            GroupUnopened => "unopened group",
            // other matches omitted for brevity
        }
    }
}

#[test]
fn test_group_unopened_description() {
    let error = ErrorKind {
        kind: ErrorKindVariant::GroupUnopened,
    };
    assert_eq!(error.description(), "unopened group");
}

