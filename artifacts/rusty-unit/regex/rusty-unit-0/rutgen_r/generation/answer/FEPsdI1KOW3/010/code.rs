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
fn test_punct_name() {
    let result = from_name("punct");
    assert_eq!(result, Some(ClassAsciiKind::Punct));
}

#[test]
fn test_invalid_names() {
    let invalid_names = [
        "alnum", "alpha", "ascii", "blank", 
        "cntrl", "digit", "graph", "lower", 
        "print"
    ];
    
    for &name in &invalid_names {
        let result = from_name(name);
        assert_eq!(result, None);
    }
}

#[test]
fn test_non_existent_name() {
    let result = from_name("non_existent");
    assert_eq!(result, None);
}

