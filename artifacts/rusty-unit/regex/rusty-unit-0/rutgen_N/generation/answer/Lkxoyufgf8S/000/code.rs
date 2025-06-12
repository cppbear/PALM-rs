// Answer 0

#[derive(Debug)]
enum ClassAsciiKind {
    Alnum,
    Alpha,
    Ascii,
    Blank,
    Cntrl,
    Digit,
    Graph,
    Lower,
    Print,
    Punct,
    Space,
    Upper,
    Word,
    Xdigit,
}

fn ascii_class(kind: &ClassAsciiKind) -> &'static [(char, char)] {
    use ClassAsciiKind::*;

    type T = &'static [(char, char)];
    match *kind {
        Alnum => {
            const X: T = &[('0', '9'), ('A', 'Z'), ('a', 'z')];
            X
        }
        Alpha => {
            const X: T = &[('A', 'Z'), ('a', 'z')];
            X
        }
        Ascii => {
            const X: T = &[('\x00', '\x7F')];
            X
        }
        Blank => {
            const X: T = &[(' ', '\t')];
            X
        }
        Cntrl => {
            const X: T = &[('\x00', '\x1F'), ('\x7F', '\x7F')];
            X
        }
        Digit => {
            const X: T = &[('0', '9')];
            X
        }
        Graph => {
            const X: T = &[('!', '~')];
            X
        }
        Lower => {
            const X: T = &[('a', 'z')];
            X
        }
        Print => {
            const X: T = &[(' ', '~')];
            X
        }
        Punct => {
            const X: T = &[('!', '/'), (':', '@'), ('[', '`'), ('{', '~')];
            X
        }
        Space => {
            const X: T = &[
                ('\t', '\t'), ('\n', '\n'), ('\x0B', '\x0B'), ('\x0C', '\x0C'),
                ('\r', '\r'), (' ', ' '),
            ];
            X
        }
        Upper => {
            const X: T = &[('A', 'Z')];
            X
        }
        Word => {
            const X: T = &[('0', '9'), ('A', 'Z'), ('_', '_'), ('a', 'z')];
            X
        }
        Xdigit => {
            const X: T = &[('0', '9'), ('A', 'F'), ('a', 'f')];
            X
        }
    }
}

#[test]
fn test_ascii_class_alnum() {
    let kind = ClassAsciiKind::Alnum;
    let result = ascii_class(&kind);
    assert_eq!(result, &[('0', '9'), ('A', 'Z'), ('a', 'z')]);
}

#[test]
fn test_ascii_class_alpha() {
    let kind = ClassAsciiKind::Alpha;
    let result = ascii_class(&kind);
    assert_eq!(result, &[('A', 'Z'), ('a', 'z')]);
}

#[test]
fn test_ascii_class_ascii() {
    let kind = ClassAsciiKind::Ascii;
    let result = ascii_class(&kind);
    assert_eq!(result, &[('\x00', '\x7F')]);
}

#[test]
fn test_ascii_class_blank() {
    let kind = ClassAsciiKind::Blank;
    let result = ascii_class(&kind);
    assert_eq!(result, &[(' ', '\t')]);
}

#[test]
fn test_ascii_class_cntrl() {
    let kind = ClassAsciiKind::Cntrl;
    let result = ascii_class(&kind);
    assert_eq!(result, &[('\x00', '\x1F'), ('\x7F', '\x7F')]);
}

#[test]
fn test_ascii_class_digit() {
    let kind = ClassAsciiKind::Digit;
    let result = ascii_class(&kind);
    assert_eq!(result, &[('0', '9')]);
}

#[test]
fn test_ascii_class_graph() {
    let kind = ClassAsciiKind::Graph;
    let result = ascii_class(&kind);
    assert_eq!(result, &[('!', '~')]);
}

#[test]
fn test_ascii_class_lower() {
    let kind = ClassAsciiKind::Lower;
    let result = ascii_class(&kind);
    assert_eq!(result, &[('a', 'z')]);
}

#[test]
fn test_ascii_class_print() {
    let kind = ClassAsciiKind::Print;
    let result = ascii_class(&kind);
    assert_eq!(result, &[(' ', '~')]);
}

#[test]
fn test_ascii_class_punct() {
    let kind = ClassAsciiKind::Punct;
    let result = ascii_class(&kind);
    assert_eq!(result, &[('!', '/'), (':', '@'), ('[', '`'), ('{', '~')]);
}

#[test]
fn test_ascii_class_space() {
    let kind = ClassAsciiKind::Space;
    let result = ascii_class(&kind);
    assert_eq!(result, &[('\t', '\t'), ('\n', '\n'), ('\x0B', '\x0B'), ('\x0C', '\x0C'), ('\r', '\r'), (' ', ' ')]);
}

#[test]
fn test_ascii_class_upper() {
    let kind = ClassAsciiKind::Upper;
    let result = ascii_class(&kind);
    assert_eq!(result, &[('A', 'Z')]);
}

#[test]
fn test_ascii_class_word() {
    let kind = ClassAsciiKind::Word;
    let result = ascii_class(&kind);
    assert_eq!(result, &[('0', '9'), ('A', 'Z'), ('_', '_'), ('a', 'z')]);
}

#[test]
fn test_ascii_class_xdigit() {
    let kind = ClassAsciiKind::Xdigit;
    let result = ascii_class(&kind);
    assert_eq!(result, &[('0', '9'), ('A', 'F'), ('a', 'f')]);
}

