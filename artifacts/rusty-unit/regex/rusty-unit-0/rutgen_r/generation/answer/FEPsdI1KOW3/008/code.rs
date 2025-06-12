// Answer 0

#[derive(Debug, PartialEq)]
pub enum ClassAsciiKind {
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

pub fn from_name(name: &str) -> Option<ClassAsciiKind> {
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
fn test_from_name_lower() {
    let result = from_name("lower");
    assert_eq!(result, Some(ClassAsciiKind::Lower));
}

#[test]
fn test_from_name_invalid1() {
    let result = from_name("alnum");
    assert_eq!(result, None);
}

#[test]
fn test_from_name_invalid2() {
    let result = from_name("alpha");
    assert_eq!(result, None);
}

#[test]
fn test_from_name_invalid3() {
    let result = from_name("ascii");
    assert_eq!(result, None);
}

#[test]
fn test_from_name_invalid4() {
    let result = from_name("blank");
    assert_eq!(result, None);
}

#[test]
fn test_from_name_invalid5() {
    let result = from_name("cntrl");
    assert_eq!(result, None);
}

#[test]
fn test_from_name_invalid6() {
    let result = from_name("digit");
    assert_eq!(result, None);
}

#[test]
fn test_from_name_invalid7() {
    let result = from_name("graph");
    assert_eq!(result, None);
}

