// Answer 0

#[test]
fn test_from_name_upper() {
    assert_eq!(ClassAsciiKind::from_name("upper"), Some(ClassAsciiKind::Upper));
}

#[test]
fn test_from_name_invalid_alnum() {
    assert_eq!(ClassAsciiKind::from_name("alnum"), None);
}

#[test]
fn test_from_name_invalid_alpha() {
    assert_eq!(ClassAsciiKind::from_name("alpha"), None);
}

#[test]
fn test_from_name_invalid_ascii() {
    assert_eq!(ClassAsciiKind::from_name("ascii"), None);
}

#[test]
fn test_from_name_invalid_blank() {
    assert_eq!(ClassAsciiKind::from_name("blank"), None);
}

#[test]
fn test_from_name_invalid_cntrl() {
    assert_eq!(ClassAsciiKind::from_name("cntrl"), None);
}

#[test]
fn test_from_name_invalid_digit() {
    assert_eq!(ClassAsciiKind::from_name("digit"), None);
}

#[test]
fn test_from_name_invalid_graph() {
    assert_eq!(ClassAsciiKind::from_name("graph"), None);
}

#[test]
fn test_from_name_invalid_lower() {
    assert_eq!(ClassAsciiKind::from_name("lower"), None);
}

#[test]
fn test_from_name_invalid_print() {
    assert_eq!(ClassAsciiKind::from_name("print"), None);
}

#[test]
fn test_from_name_invalid_punct() {
    assert_eq!(ClassAsciiKind::from_name("punct"), None);
}

#[test]
fn test_from_name_invalid_space() {
    assert_eq!(ClassAsciiKind::from_name("space"), None);
}

#[test]
fn test_from_name_invalid_xdigit() {
    assert_eq!(ClassAsciiKind::from_name("xdigit"), None);
}

#[test]
fn test_from_name_invalid_random() {
    assert_eq!(ClassAsciiKind::from_name("random"), None);
}

