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
fn test_from_name_blank() {
    assert_eq!(from_name("blank"), Some(ClassAsciiKind::Blank));
}

#[test]
fn test_from_name_missing() {
    assert_eq!(from_name("not_a_variant"), None);
}

#[test]
fn test_from_name_alnum() {
    assert_eq!(from_name("alnum"), None);
}

#[test]
fn test_from_name_alpha() {
    assert_eq!(from_name("alpha"), None);
}

#[test]
fn test_from_name_ascii() {
    assert_eq!(from_name("ascii"), None);
}

