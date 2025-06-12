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
fn test_from_name_space() {
    let result = from_name("space");
    assert_eq!(result, Some(ClassAsciiKind::Space));
}

#[test]
fn test_from_name_invalid() {
    let names = [
        "alnum", "alpha", "ascii", "blank", "cntrl", 
        "digit", "graph", "lower", "print", "punct", 
        "upper", "word", "xdigit"
    ];
    for &name in &names {
        let result = from_name(name);
        assert_eq!(result, None);
    }
}

