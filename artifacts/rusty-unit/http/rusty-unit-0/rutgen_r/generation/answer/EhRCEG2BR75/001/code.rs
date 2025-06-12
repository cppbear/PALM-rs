// Answer 0


use std::fmt;

struct InvalidHeaderName;

impl fmt::Debug for InvalidHeaderName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("InvalidHeaderName")
            .finish()
    }
}

#[test]
fn test_invalid_header_name_debug_format() {
    let header_name = InvalidHeaderName;
    let formatted_str = format!("{:?}", header_name);
    assert_eq!(formatted_str, "InvalidHeaderName");
}

#[test]
fn test_invalid_header_name_debug_format_empty() {
    let header_name = InvalidHeaderName;
    let result = std::panic::catch_unwind(|| {
        let _ = format!("{:?}", header_name);
    });
    assert!(result.is_ok());
}


