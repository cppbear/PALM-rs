// Answer 0

#[derive(Debug)]
enum Error {
    Parse(String),
    Translate(String),
    Other,
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
#[should_panic]
fn test_fmt_unreachable_other() {
    let err = Error::Other;
    let _ = format!("{}", err);
}

#[test]
#[should_panic]
fn test_fmt_unreachable_parse() {
    let err = Error::Parse("parse error".to_string());
    let _ = format!("{}", err);
}

#[test]
#[should_panic]
fn test_fmt_unreachable_translate() {
    let err = Error::Translate("translate error".to_string());
    let _ = format!("{}", err);
}

