// Answer 0

#[derive(Debug)]
enum Error {
    Parse(String),
    Translate(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::Parse(ref x) => x.fmt(f),
            Error::Translate(ref x) => x.fmt(f),
            _ => unreachable!(),
        }
    }
}

#[test]
fn test_fmt_parse_error() {
    let parse_error = Error::Parse(String::from("parse error"));
    let mut output = String::new();
    let result = write!(&mut output, "{}", parse_error);
    assert!(result.is_ok());
    assert_eq!(output, "parse error");
}

#[test]
fn test_fmt_translate_error() {
    let translate_error = Error::Translate(String::from("translate error"));
    let mut output = String::new();
    let result = write!(&mut output, "{}", translate_error);
    assert!(result.is_ok());
    assert_eq!(output, "translate error");
}

