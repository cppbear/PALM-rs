// Answer 0

#[derive(Debug)]
struct InvalidUri;

#[derive(Debug)]
struct Authority;

impl Authority {
    fn parse(s: &[u8]) -> Result<usize, InvalidUri> {
        if s.len() > 0 {
            Ok(s.len())
        } else {
            Err(InvalidUri)
        }
    }
}

#[derive(Debug)]
enum ErrorKind {
    Empty,
}

impl From<ErrorKind> for InvalidUri {
    fn from(_: ErrorKind) -> Self {
        InvalidUri
    }
}

fn parse_non_empty(s: &[u8]) -> Result<usize, InvalidUri> {
    if s.is_empty() {
        return Err(ErrorKind::Empty.into());
    }
    Authority::parse(s)
}

#[test]
fn test_parse_non_empty_valid() {
    let input = b"valid_input";
    let result = parse_non_empty(input);
    assert_eq!(result, Ok(input.len()));
}

#[test]
#[should_panic]
fn test_parse_non_empty_empty() {
    let input: &[u8] = b"";
    let result = parse_non_empty(input);
    assert!(result.is_err());
}

