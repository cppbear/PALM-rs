// Answer 0

#[test]
fn test_from_name_punct() {
    let result = ClassAsciiKind::from_name("punct");
}

#[test]
fn test_from_name_invalid_name() {
    let result = ClassAsciiKind::from_name("not_a_kind");
}

#[test]
fn test_from_name_empty_string() {
    let result = ClassAsciiKind::from_name("");
}

#[test]
fn test_from_name_alnum() {
    let result = ClassAsciiKind::from_name("alnum");
}

#[test]
fn test_from_name_alpha() {
    let result = ClassAsciiKind::from_name("alpha");
}

#[test]
fn test_from_name_ascii() {
    let result = ClassAsciiKind::from_name("ascii");
}

#[test]
fn test_from_name_blank() {
    let result = ClassAsciiKind::from_name("blank");
}

#[test]
fn test_from_name_cntrl() {
    let result = ClassAsciiKind::from_name("cntrl");
}

#[test]
fn test_from_name_digit() {
    let result = ClassAsciiKind::from_name("digit");
}

#[test]
fn test_from_name_graph() {
    let result = ClassAsciiKind::from_name("graph");
}

#[test]
fn test_from_name_lower() {
    let result = ClassAsciiKind::from_name("lower");
}

#[test]
fn test_from_name_print() {
    let result = ClassAsciiKind::from_name("print");
}

#[test]
fn test_from_name_space() {
    let result = ClassAsciiKind::from_name("space");
}

#[test]
fn test_from_name_upper() {
    let result = ClassAsciiKind::from_name("upper");
}

#[test]
fn test_from_name_word() {
    let result = ClassAsciiKind::from_name("word");
}

#[test]
fn test_from_name_xdigit() {
    let result = ClassAsciiKind::from_name("xdigit");
}

