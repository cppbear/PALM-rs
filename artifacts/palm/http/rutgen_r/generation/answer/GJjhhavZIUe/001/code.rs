// Answer 0


#[derive(Debug)]
struct Error {
    message: String,
    kind: String,
}

impl Error {
    fn get_ref(&self) -> &String {
        &self.message
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("http::Error")
            .field(&self.get_ref())
            .finish()
    }
}

#[test]
fn test_error_fmt_valid() {
    let error = Error {
        message: "Not Found".to_string(),
        kind: "Client".to_string(),
    };

    let formatted = format!("{:?}", error);
    assert!(formatted.contains("http::Error"));
    assert!(formatted.contains("Not Found"));
}

#[test]
fn test_error_fmt_empty_message() {
    let error = Error {
        message: "".to_string(),
        kind: "Client".to_string(),
    };

    let formatted = format!("{:?}", error);
    assert!(formatted.contains("http::Error"));
    assert!(formatted.contains(""));
}

#[test]
fn test_error_fmt_special_characters() {
    let error = Error {
        message: "Error: \n\tUnexpected character '@'".to_string(),
        kind: "Server".to_string(),
    };

    let formatted = format!("{:?}", error);
    assert!(formatted.contains("http::Error"));
    assert!(formatted.contains("Error: \n\tUnexpected character '@'"));
}


