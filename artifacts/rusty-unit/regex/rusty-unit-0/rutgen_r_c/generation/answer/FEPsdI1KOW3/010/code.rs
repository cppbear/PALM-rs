// Answer 0

#[test]
fn test_from_name_punct() {
    let result = ClassAsciiKind::from_name("punct");
    assert_eq!(result, Some(ClassAsciiKind::Punct));
}

#[test]
fn test_from_name_invalid_alnum() {
    let result = ClassAsciiKind::from_name("alnum");
    assert_eq!(result, None);
}

#[test]
fn test_from_name_invalid_alpha() {
    let result = ClassAsciiKind::from_name("alpha");
    assert_eq!(result, None);
}

#[test]
fn test_from_name_invalid_ascii() {
    let result = ClassAsciiKind::from_name("ascii");
    assert_eq!(result, None);
}

#[test]
fn test_from_name_invalid_blank() {
    let result = ClassAsciiKind::from_name("blank");
    assert_eq!(result, None);
}

#[test]
fn test_from_name_invalid_cntrl() {
    let result = ClassAsciiKind::from_name("cntrl");
    assert_eq!(result, None);
}

#[test]
fn test_from_name_invalid_digit() {
    let result = ClassAsciiKind::from_name("digit");
    assert_eq!(result, None);
}

#[test]
fn test_from_name_invalid_graph() {
    let result = ClassAsciiKind::from_name("graph");
    assert_eq!(result, None);
}

#[test]
fn test_from_name_invalid_lower() {
    let result = ClassAsciiKind::from_name("lower");
    assert_eq!(result, None);
}

#[test]
fn test_from_name_invalid_print() {
    let result = ClassAsciiKind::from_name("print");
    assert_eq!(result, None);
}

#[test]
fn test_from_name_invalid_space() {
    let result = ClassAsciiKind::from_name("space");
    assert_eq!(result, None);
}

#[test]
fn test_from_name_invalid_upper() {
    let result = ClassAsciiKind::from_name("upper");
    assert_eq!(result, None);
}

#[test]
fn test_from_name_invalid_word() {
    let result = ClassAsciiKind::from_name("word");
    assert_eq!(result, None);
}

#[test]
fn test_from_name_invalid_xdigit() {
    let result = ClassAsciiKind::from_name("xdigit");
    assert_eq!(result, None);
}

