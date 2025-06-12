// Answer 0

#[test]
fn test_from_name_digit() {
    let name = "digit";
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
fn test_from_name_invalid_print() {
    let name = "print";
    let result = ClassAsciiKind::from_name(name);
}

#[test]
fn test_from_name_invalid_punct() {
    let name = "punct";
    let result = ClassAsciiKind::from_name(name);
}

#[test]
fn test_from_name_invalid_space() {
    let name = "space";
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
fn test_from_name_invalid_unknown() {
    let name = "unknown";
    let result = ClassAsciiKind::from_name(name);
}

