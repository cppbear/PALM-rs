// Answer 0

#[test]
fn test_ascii_class_print() {
    use ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Print;
    let expected: &[(char, char)] = &[(' ', '~')];
    let result = ascii_class(&kind);
    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_digit() {
    use ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Digit;
    let expected: &[(char, char)] = &[('0', '9')];
    let result = ascii_class(&kind);
    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_alpha() {
    use ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Alpha;
    let expected: &[(char, char)] = &[('A', 'Z'), ('a', 'z')];
    let result = ascii_class(&kind);
    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_alnum() {
    use ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Alnum;
    let expected: &[(char, char)] = &[('0', '9'), ('A', 'Z'), ('a', 'z')];
    let result = ascii_class(&kind);
    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_space() {
    use ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Space;
    let expected: &[(char, char)] = &[
        ('\t', '\t'), 
        ('\n', '\n'), 
        ('\x0B', '\x0B'), 
        ('\x0C', '\x0C'),
        ('\r', '\r'), 
        (' ', ' '),
    ];
    let result = ascii_class(&kind);
    assert_eq!(result, expected);
}

