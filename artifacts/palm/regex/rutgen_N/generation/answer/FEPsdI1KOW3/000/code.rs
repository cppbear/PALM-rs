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
    use ClassAsciiKind::*;
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
fn test_from_name_valid_variants() {
    assert_eq!(from_name("alnum"), Some(ClassAsciiKind::Alnum));
    assert_eq!(from_name("alpha"), Some(ClassAsciiKind::Alpha));
    assert_eq!(from_name("ascii"), Some(ClassAsciiKind::Ascii));
    assert_eq!(from_name("blank"), Some(ClassAsciiKind::Blank));
    assert_eq!(from_name("cntrl"), Some(ClassAsciiKind::Cntrl));
    assert_eq!(from_name("digit"), Some(ClassAsciiKind::Digit));
    assert_eq!(from_name("graph"), Some(ClassAsciiKind::Graph));
    assert_eq!(from_name("lower"), Some(ClassAsciiKind::Lower));
    assert_eq!(from_name("print"), Some(ClassAsciiKind::Print));
    assert_eq!(from_name("punct"), Some(ClassAsciiKind::Punct));
    assert_eq!(from_name("space"), Some(ClassAsciiKind::Space));
    assert_eq!(from_name("upper"), Some(ClassAsciiKind::Upper));
    assert_eq!(from_name("word"), Some(ClassAsciiKind::Word));
    assert_eq!(from_name("xdigit"), Some(ClassAsciiKind::Xdigit));
}

#[test]
fn test_from_name_invalid_variant() {
    assert_eq!(from_name("unknown"), None);
    assert_eq!(from_name("alphanumeric"), None);
    assert_eq!(from_name(""), None);
}

