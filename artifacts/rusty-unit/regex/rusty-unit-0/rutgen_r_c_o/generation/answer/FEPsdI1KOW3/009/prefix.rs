// Answer 0

#[test]
fn test_from_name_valid_print() {
    let name = "print";
    let result = ClassAsciiKind::from_name(name);
}

#[test]
fn test_from_name_invalid_alnum() {
    let name = "alnum";
    let result = ClassAsciiKind::from_name(name);
}

#[test]
fn test_from_name_invalid_alpha() {
    let name = "alpha";
    let result = ClassAsciiKind::from_name(name);
}

#[test]
fn test_from_name_invalid_ascii() {
    let name = "ascii";
    let result = ClassAsciiKind::from_name(name);
}

#[test]
fn test_from_name_invalid_blank() {
    let name = "blank";
    let result = ClassAsciiKind::from_name(name);
}

#[test]
fn test_from_name_invalid_cntrl() {
    let name = "cntrl";
    let result = ClassAsciiKind::from_name(name);
}

#[test]
fn test_from_name_invalid_digit() {
    let name = "digit";
    let result = ClassAsciiKind::from_name(name);
}

#[test]
fn test_from_name_invalid_graph() {
    let name = "graph";
    let result = ClassAsciiKind::from_name(name);
}

#[test]
fn test_from_name_invalid_lower() {
    let name = "lower";
    let result = ClassAsciiKind::from_name(name);
}

#[test]
fn test_from_name_invalid_upper() {
    let name = "upper";
    let result = ClassAsciiKind::from_name(name);
}

#[test]
fn test_from_name_invalid_word() {
    let name = "word";
    let result = ClassAsciiKind::from_name(name);
}

#[test]
fn test_from_name_invalid_xdigit() {
    let name = "xdigit";
    let result = ClassAsciiKind::from_name(name);
}

#[test]
fn test_from_name_invalid_other() {
    let name = "not_a_valid_name";
    let result = ClassAsciiKind::from_name(name);
}

