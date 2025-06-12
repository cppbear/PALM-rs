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
fn test_from_name_print() {
    assert_eq!(from_name("print"), Some(ClassAsciiKind::Print));
}

#[test]
fn test_from_name_invalid_cases() {
    assert_eq!(from_name("alnum"), None);
    assert_eq!(from_name("alpha"), None);
    assert_eq!(from_name("ascii"), None);
    assert_eq!(from_name("blank"), None);
    assert_eq!(from_name("cntrl"), None);
    assert_eq!(from_name("digit"), None);
    assert_eq!(from_name("graph"), None);
    assert_eq!(from_name("lower"), None);
}

