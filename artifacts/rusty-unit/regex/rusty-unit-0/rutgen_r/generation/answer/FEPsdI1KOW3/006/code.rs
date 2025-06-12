// Answer 0

#[test]
fn test_from_name_digit() {
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

    assert_eq!(from_name("digit"), Some(ClassAsciiKind::Digit));
} 

#[test]
fn test_from_name_invalid() {
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

    let invalid_names = vec!["alnum", "alpha", "ascii", "blank", "cntrl"];
    for name in invalid_names {
        assert_eq!(from_name(name), None);
    }
}

