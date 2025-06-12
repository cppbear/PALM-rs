// Answer 0

#[test]
fn test_from_name_alnum() {
    assert_eq!(ClassAsciiKind::from_name("alnum"), Some(ClassAsciiKind::Alnum));
}

#[test]
fn test_from_name_alpha() {
    assert_eq!(ClassAsciiKind::from_name("alpha"), Some(ClassAsciiKind::Alpha));
}

#[test]
fn test_from_name_ascii() {
    assert_eq!(ClassAsciiKind::from_name("ascii"), Some(ClassAsciiKind::Ascii));
}

#[test]
fn test_from_name_blank() {
    assert_eq!(ClassAsciiKind::from_name("blank"), Some(ClassAsciiKind::Blank));
}

#[test]
fn test_from_name_cntrl() {
    assert_eq!(ClassAsciiKind::from_name("cntrl"), Some(ClassAsciiKind::Cntrl));
}

#[test]
fn test_from_name_digit() {
    assert_eq!(ClassAsciiKind::from_name("digit"), Some(ClassAsciiKind::Digit));
}

#[test]
fn test_from_name_graph() {
    assert_eq!(ClassAsciiKind::from_name("graph"), Some(ClassAsciiKind::Graph));
}

#[test]
fn test_from_name_lower() {
    assert_eq!(ClassAsciiKind::from_name("lower"), Some(ClassAsciiKind::Lower));
}

#[test]
fn test_from_name_print() {
    assert_eq!(ClassAsciiKind::from_name("print"), Some(ClassAsciiKind::Print));
}

#[test]
fn test_from_name_punct() {
    assert_eq!(ClassAsciiKind::from_name("punct"), Some(ClassAsciiKind::Punct));
}

#[test]
fn test_from_name_space() {
    assert_eq!(ClassAsciiKind::from_name("space"), Some(ClassAsciiKind::Space));
}

#[test]
fn test_from_name_upper() {
    assert_eq!(ClassAsciiKind::from_name("upper"), Some(ClassAsciiKind::Upper));
}

#[test]
fn test_from_name_word() {
    assert_eq!(ClassAsciiKind::from_name("word"), Some(ClassAsciiKind::Word));
}

#[test]
fn test_from_name_xdigit() {
    assert_eq!(ClassAsciiKind::from_name("xdigit"), Some(ClassAsciiKind::Xdigit));
}

#[test]
fn test_from_name_invalid() {
    assert_eq!(ClassAsciiKind::from_name("unknown"), None);
}

#[test]
fn test_from_name_empty() {
    assert_eq!(ClassAsciiKind::from_name(""), None);
}

