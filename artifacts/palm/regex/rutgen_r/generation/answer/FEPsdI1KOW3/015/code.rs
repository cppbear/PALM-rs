// Answer 0

#[derive(Debug, PartialEq)]
enum ClassAsciiKind {
    Alnum,
    Alpha,
    Ascii,
    Blank,
    Cntrl,
    Digit,
    Graph,
    Lower,
    Print,
    Punct,
    Space,
    Upper,
    Word,
    Xdigit,
}

fn from_name(name: &str) -> Option<ClassAsciiKind> {
    use self::ClassAsciiKind::*;
    match name {
        "alnum" => Some(Alnum),
        "alpha" => Some(Alpha),
        "ascii" => Some(Ascii),
        "blank" => Some(Blank),
        "cntrl" => Some(Cntrl),
        "digit" => Some(Digit),
        "graph" => Some(Graph),
        "lower" => Some(Lower),
        "print" => Some(Print),
        "punct" => Some(Punct),
        "space" => Some(Space),
        "upper" => Some(Upper),
        "word" => Some(Word),
        "xdigit" => Some(Xdigit),
        _ => None,
    }
}

#[test]
fn test_from_name_none_cases() {
    assert_eq!(from_name("notavalidname"), None);
    assert_eq!(from_name("12345"), None);
    assert_eq!(from_name("!@#$%"), None);
    assert_eq!(from_name("randomString"), None);
    assert_eq!(from_name("lowercase"), None);
    assert_eq!(from_name("UPPERCASE"), None);
    assert_eq!(from_name(""), None);
    assert_eq!(from_name("unknown"), None);
    assert_eq!(from_name("alnum1"), None);
    assert_eq!(from_name("alpha!"), None);
}

