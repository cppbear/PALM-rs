// Answer 0

#[derive(Debug)]
enum Error {
    EmptyRange,
    NonFinite,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Error::EmptyRange => "low > high (or equal if exclusive) in uniform distribution",
            Error::NonFinite => "Non-finite range in uniform distribution",
        })
    }
}

#[test]
fn test_error_empty_range() {
    let error = Error::EmptyRange;
    let result = format!("{}", error);
    assert_eq!(result, "low > high (or equal if exclusive) in uniform distribution");
}

#[test]
fn test_error_non_finite() {
    let error = Error::NonFinite;
    let result = format!("{}", error);
    assert_eq!(result, "Non-finite range in uniform distribution");
}

