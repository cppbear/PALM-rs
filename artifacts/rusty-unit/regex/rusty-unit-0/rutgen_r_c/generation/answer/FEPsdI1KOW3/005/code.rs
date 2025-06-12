// Answer 0

#[test]
fn test_from_name_cntrl() {
    let name = "cntrl";
    let result = ClassAsciiKind::from_name(name);
    assert_eq!(result, Some(ClassAsciiKind::Cntrl));
}

#[test]
fn test_from_name_alnum_false() {
    let name = "alnum";
    let result = ClassAsciiKind::from_name(name);
    assert_eq!(result, None);
}

#[test]
fn test_from_name_alpha_false() {
    let name = "alpha";
    let result = ClassAsciiKind::from_name(name);
    assert_eq!(result, None);
}

#[test]
fn test_from_name_ascii_false() {
    let name = "ascii";
    let result = ClassAsciiKind::from_name(name);
    assert_eq!(result, None);
}

#[test]
fn test_from_name_blank_false() {
    let name = "blank";
    let result = ClassAsciiKind::from_name(name);
    assert_eq!(result, None);
}

