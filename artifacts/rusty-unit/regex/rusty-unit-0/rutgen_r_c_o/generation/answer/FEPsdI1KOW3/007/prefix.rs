// Answer 0

#[test]
fn test_from_name_graph() {
    let name = "graph";
    let result = ClassAsciiKind::from_name(name);
}

#[test]
fn test_from_name_invalid_case() {
    let name = "invalid_case";
    let result = ClassAsciiKind::from_name(name);
}

#[test]
fn test_from_name_empty_string() {
    let name = "";
    let result = ClassAsciiKind::from_name(name);
}

#[test]
fn test_from_name_lowercase() {
    let name = "lower";
    let result = ClassAsciiKind::from_name(name);
}

#[test]
fn test_from_name_uppercase() {
    let name = "UPPER";
    let result = ClassAsciiKind::from_name(name);
}

