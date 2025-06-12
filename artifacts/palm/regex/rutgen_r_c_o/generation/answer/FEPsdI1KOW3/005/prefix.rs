// Answer 0

#[test]
fn test_from_name_cntrl() {
    let result = ClassAsciiKind::from_name("cntrl");
}

#[test]
fn test_from_name_not_alnum() {
    let result = ClassAsciiKind::from_name("alnum");
}

#[test]
fn test_from_name_not_alpha() {
    let result = ClassAsciiKind::from_name("alpha");
}

#[test]
fn test_from_name_not_ascii() {
    let result = ClassAsciiKind::from_name("ascii");
}

#[test]
fn test_from_name_not_blank() {
    let result = ClassAsciiKind::from_name("blank");
}

#[test]
fn test_from_name_not_digit() {
    let result = ClassAsciiKind::from_name("digit");
}

#[test]
fn test_from_name_not_graph() {
    let result = ClassAsciiKind::from_name("graph");
}

#[test]
fn test_from_name_not_lower() {
    let result = ClassAsciiKind::from_name("lower");
}

#[test]
fn test_from_name_not_print() {
    let result = ClassAsciiKind::from_name("print");
}

#[test]
fn test_from_name_not_punct() {
    let result = ClassAsciiKind::from_name("punct");
}

#[test]
fn test_from_name_not_space() {
    let result = ClassAsciiKind::from_name("space");
}

#[test]
fn test_from_name_not_upper() {
    let result = ClassAsciiKind::from_name("upper");
}

#[test]
fn test_from_name_not_word() {
    let result = ClassAsciiKind::from_name("word");
}

#[test]
fn test_from_name_not_xdigit() {
    let result = ClassAsciiKind::from_name("xdigit");
}

#[test]
fn test_from_name_invalid() {
    let result = ClassAsciiKind::from_name("invalid");
}

