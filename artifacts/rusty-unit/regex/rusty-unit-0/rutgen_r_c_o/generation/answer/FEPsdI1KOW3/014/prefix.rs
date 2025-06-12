// Answer 0

#[test]
fn test_from_name_xdigit() {
    let result = ClassAsciiKind::from_name("xdigit");
}

#[test]
fn test_from_name_invalid_alnum() {
    let result = ClassAsciiKind::from_name("alnum");
}

#[test]
fn test_from_name_invalid_alpha() {
    let result = ClassAsciiKind::from_name("alpha");
}

#[test]
fn test_from_name_invalid_ascii() {
    let result = ClassAsciiKind::from_name("ascii");
}

#[test]
fn test_from_name_invalid_blank() {
    let result = ClassAsciiKind::from_name("blank");
}

#[test]
fn test_from_name_invalid_cntrl() {
    let result = ClassAsciiKind::from_name("cntrl");
}

#[test]
fn test_from_name_invalid_digit() {
    let result = ClassAsciiKind::from_name("digit");
}

#[test]
fn test_from_name_invalid_graph() {
    let result = ClassAsciiKind::from_name("graph");
}

#[test]
fn test_from_name_invalid_lower() {
    let result = ClassAsciiKind::from_name("lower");
}

#[test]
fn test_from_name_invalid_print() {
    let result = ClassAsciiKind::from_name("print");
}

#[test]
fn test_from_name_invalid_punct() {
    let result = ClassAsciiKind::from_name("punct");
}

#[test]
fn test_from_name_invalid_space() {
    let result = ClassAsciiKind::from_name("space");
}

#[test]
fn test_from_name_invalid_upper() {
    let result = ClassAsciiKind::from_name("upper");
}

#[test]
fn test_from_name_invalid_word() {
    let result = ClassAsciiKind::from_name("word");
}

