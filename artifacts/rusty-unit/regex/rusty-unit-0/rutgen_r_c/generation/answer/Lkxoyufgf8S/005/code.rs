// Answer 0

#[test]
fn test_ascii_class_punct() {
    use ast::ClassAsciiKind::*;

    let kind = Punct;
    let result = ascii_class(&kind);
    let expected: &[(char, char)] = &[('!', '/'), (':', '@'), ('[', '`'), ('{', '~')];

    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_graph() {
    use ast::ClassAsciiKind::*;

    let kind = Graph;
    let result = ascii_class(&kind);
    let expected: &[(char, char)] = &[('!', '~')];

    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_space() {
    use ast::ClassAsciiKind::*;

    let kind = Space;
    let result = ascii_class(&kind);
    let expected: &[(char, char)] = &[
        ('\t', '\t'), ('\n', '\n'), ('\x0B', '\x0B'), ('\x0C', '\x0C'),
        ('\r', '\r'), (' ', ' '),
    ];

    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_lower() {
    use ast::ClassAsciiKind::*;

    let kind = Lower;
    let result = ascii_class(&kind);
    let expected: &[(char, char)] = &[('a', 'z')];

    assert_eq!(result, expected);
}

