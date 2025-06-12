// Answer 0

#[derive(Debug)]
enum Content {
    Str(String),
    // other variants...
}

#[derive(Debug)]
enum Unexpected {
    Str(String),
    // other variants...
}

impl Content {
    fn unexpected(&self) -> Unexpected {
        match *self {
            Content::Str(ref s) => Unexpected::Str(s.clone()),
            // other matches...
        }
    }
}

#[test]
fn test_unexpected_str() {
    let content = Content::Str(String::from("test string"));
    let unexpected = content.unexpected();
    if let Unexpected::Str(ref value) = unexpected {
        assert_eq!(value, "test string");
    } else {
        panic!("Expected Unexpected::Str for the input Content::Str");
    }
}

#[test]
fn test_unexpected_str_boundary() {
    let content = Content::Str(String::from(""));
    let unexpected = content.unexpected();
    if let Unexpected::Str(ref value) = unexpected {
        assert_eq!(value, "");
    } else {
        panic!("Expected Unexpected::Str for the input Content::Str with empty string");
    }
}

