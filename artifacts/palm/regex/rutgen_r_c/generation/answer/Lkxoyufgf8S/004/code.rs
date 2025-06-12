// Answer 0

#[test]
fn test_ascii_class_blank() {
    use ast::ClassAsciiKind;

    let result = ascii_class(&ClassAsciiKind::Blank);
    let expected: &[(char, char)] = &[(' ', '\t')];
    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_space() {
    use ast::ClassAsciiKind;

    let result = ascii_class(&ClassAsciiKind::Space);
    let expected: &[(char, char)] = &[
        ('\t', '\t'), 
        ('\n', '\n'), 
        ('\x0B', '\x0B'), 
        ('\x0C', '\x0C'), 
        ('\r', '\r'), 
        (' ', ' '),
    ];
    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_cntrl() {
    use ast::ClassAsciiKind;

    let result = ascii_class(&ClassAsciiKind::Cntrl);
    let expected: &[(char, char)] = &[('\x00', '\x1F'), ('\x7F', '\x7F')];
    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_digit() {
    use ast::ClassAsciiKind;

    let result = ascii_class(&ClassAsciiKind::Digit);
    let expected: &[(char, char)] = &[('0', '9')];
    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_upper() {
    use ast::ClassAsciiKind;

    let result = ascii_class(&ClassAsciiKind::Upper);
    let expected: &[(char, char)] = &[('A', 'Z')];
    assert_eq!(result, expected);
}

