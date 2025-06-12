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
fn test_from_name_ascii() {
    let result = from_name("ascii");
    assert_eq!(result, Some(ClassAsciiKind::Ascii));
}

#[test]
fn test_from_name_alnum_false() {
    let result = from_name("alnum");
    assert_eq!(result, None);
}

#[test]
fn test_from_name_alpha_false() {
    let result = from_name("alpha");
    assert_eq!(result, None);
}

