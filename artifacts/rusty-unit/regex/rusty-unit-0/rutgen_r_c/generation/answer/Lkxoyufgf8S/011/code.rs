// Answer 0

#[test]
fn test_asci_class_alnum() {
    use ast::ClassAsciiKind::*;
    let result = ascii_class(&Alnum);
    let expected = &[('0', '9'), ('A', 'Z'), ('a', 'z')];

    assert_eq!(result, expected);
}

#[test]
fn test_asci_class_alpha() {
    use ast::ClassAsciiKind::*;
    let result = ascii_class(&Alpha);
    let expected = &[('A', 'Z'), ('a', 'z')];

    assert_eq!(result, expected);
}

#[test]
fn test_asci_class_ascii() {
    use ast::ClassAsciiKind::*;
    let result = ascii_class(&Ascii);
    let expected = &[('\x00', '\x7F')];

    assert_eq!(result, expected);
}

#[test]
fn test_asci_class_blank() {
    use ast::ClassAsciiKind::*;
    let result = ascii_class(&Blank);
    let expected = &[(' ', '\t')];

    assert_eq!(result, expected);
}

#[test]
fn test_asci_class_cntrl() {
    use ast::ClassAsciiKind::*;
    let result = ascii_class(&Cntrl);
    let expected = &[('\x00', '\x1F'), ('\x7F', '\x7F')];

    assert_eq!(result, expected);
}

#[test]
fn test_asci_class_digit() {
    use ast::ClassAsciiKind::*;
    let result = ascii_class(&Digit);
    let expected = &[('0', '9')];

    assert_eq!(result, expected);
}

#[test]
fn test_asci_class_graph() {
    use ast::ClassAsciiKind::*;
    let result = ascii_class(&Graph);
    let expected = &[('!', '~')];

    assert_eq!(result, expected);
}

#[test]
fn test_asci_class_lower() {
    use ast::ClassAsciiKind::*;
    let result = ascii_class(&Lower);
    let expected = &[('a', 'z')];

    assert_eq!(result, expected);
}

#[test]
fn test_asci_class_print() {
    use ast::ClassAsciiKind::*;
    let result = ascii_class(&Print);
    let expected = &[(' ', '~')];

    assert_eq!(result, expected);
}

#[test]
fn test_asci_class_punct() {
    use ast::ClassAsciiKind::*;
    let result = ascii_class(&Punct);
    let expected = &[('!', '/'), (':', '@'), ('[', '`'), ('{', '~')];

    assert_eq!(result, expected);
}

#[test]
fn test_asci_class_space() {
    use ast::ClassAsciiKind::*;
    let result = ascii_class(&Space);
    let expected = &[
        ('\t', '\t'), ('\n', '\n'), ('\x0B', '\x0B'), 
        ('\x0C', '\x0C'), ('\r', '\r'), (' ', ' ')
    ];

    assert_eq!(result, expected);
}

#[test]
fn test_asci_class_upper() {
    use ast::ClassAsciiKind::*;
    let result = ascii_class(&Upper);
    let expected = &[('A', 'Z')];

    assert_eq!(result, expected);
}

#[test]
fn test_asci_class_word() {
    use ast::ClassAsciiKind::*;
    let result = ascii_class(&Word);
    let expected = &[('0', '9'), ('A', 'Z'), ('_', '_'), ('a', 'z')];

    assert_eq!(result, expected);
}

#[test]
fn test_asci_class_xdigit() {
    use ast::ClassAsciiKind::*;
    let result = ascii_class(&Xdigit);
    let expected = &[('0', '9'), ('A', 'F'), ('a', 'f')];

    assert_eq!(result, expected);
}

