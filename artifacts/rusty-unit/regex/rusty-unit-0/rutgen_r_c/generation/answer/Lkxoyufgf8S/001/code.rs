// Answer 0

#[test]
fn test_ascii_class_xdigit() {
    use ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Xdigit;
    let result = ascii_class(&kind);
    let expected: &[(char, char)] = &[('0', '9'), ('A', 'F'), ('a', 'f')];

    assert_eq!(result, expected);
}

