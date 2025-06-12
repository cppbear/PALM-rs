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
fn test_from_name_upper() {
    let result = from_name("upper");
    assert_eq!(result, Some(ClassAsciiKind::Upper));
}

#[test]
fn test_from_name_invalid() {
    let result = from_name("invalid");
    assert_eq!(result, None);
}

#[test]
fn test_from_name_other_variants() {
    let invalid_variants = [
        "alnum", "alpha", "ascii", "blank", "cntrl",
        "digit", "graph", "lower", "print", "punct",
        "space", "word", "xdigit"
    ];
    
    for &variant in &invalid_variants {
        let result = from_name(variant);
        assert_eq!(result, None);
    }
}

