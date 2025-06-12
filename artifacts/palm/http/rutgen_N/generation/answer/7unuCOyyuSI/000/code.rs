// Answer 0

#[derive(Debug)]
struct InvalidHeaderValue;

impl std::fmt::Debug for InvalidHeaderValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InvalidHeaderValue")
            .finish()
    }
}

#[test]
fn test_fmt_invalid_header_value() {
    let value = InvalidHeaderValue;
    let result = format!("{:?}", value);
    assert_eq!(result, "InvalidHeaderValue");
}

