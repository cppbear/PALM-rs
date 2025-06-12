// Answer 0

#[test]
fn test_from_name_alnum() {
    let result = ClassAsciiKind::from_name("alnum");
    assert_eq!(result, Some(ClassAsciiKind::Alnum));
}

#[test]
fn test_from_name_alpha() {
    let result = ClassAsciiKind::from_name("alpha");
    assert_eq!(result, Some(ClassAsciiKind::Alpha));
}

#[test]
fn test_from_name_ascii() {
    let result = ClassAsciiKind::from_name("ascii");
    assert_eq!(result, Some(ClassAsciiKind::Ascii));
}

#[test]
fn test_from_name_blank() {
    let result = ClassAsciiKind::from_name("blank");
    assert_eq!(result, Some(ClassAsciiKind::Blank));
}

#[test]
fn test_from_name_cntrl() {
    let result = ClassAsciiKind::from_name("cntrl");
    assert_eq!(result, Some(ClassAsciiKind::Cntrl));
}

#[test]
fn test_from_name_digit() {
    let result = ClassAsciiKind::from_name("digit");
    assert_eq!(result, Some(ClassAsciiKind::Digit));
}

#[test]
fn test_from_name_graph() {
    let result = ClassAsciiKind::from_name("graph");
    assert_eq!(result, Some(ClassAsciiKind::Graph));
}

#[test]
fn test_from_name_lower() {
    let result = ClassAsciiKind::from_name("lower");
    assert_eq!(result, Some(ClassAsciiKind::Lower));
}

#[test]
fn test_from_name_print() {
    let result = ClassAsciiKind::from_name("print");
    assert_eq!(result, Some(ClassAsciiKind::Print));
}

#[test]
fn test_from_name_punct() {
    let result = ClassAsciiKind::from_name("punct");
    assert_eq!(result, Some(ClassAsciiKind::Punct));
}

#[test]
fn test_from_name_space() {
    let result = ClassAsciiKind::from_name("space");
    assert_eq!(result, Some(ClassAsciiKind::Space));
}

#[test]
fn test_from_name_upper() {
    let result = ClassAsciiKind::from_name("upper");
    assert_eq!(result, Some(ClassAsciiKind::Upper));
}

#[test]
fn test_from_name_word() {
    let result = ClassAsciiKind::from_name("word");
    assert_eq!(result, Some(ClassAsciiKind::Word));
}

#[test]
fn test_from_name_xdigit() {
    let result = ClassAsciiKind::from_name("xdigit");
    assert_eq!(result, Some(ClassAsciiKind::Xdigit));
}

#[test]
fn test_from_name_invalid() {
    let result = ClassAsciiKind::from_name("invalid");
    assert_eq!(result, None);
}

