// Answer 0

#[test]
fn test_ascii_class_word() {
    mod ast {
        pub enum ClassAsciiKind {
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
    }

    fn ascii_class(kind: &ast::ClassAsciiKind) -> &'static [(char, char)] {
        use ast::ClassAsciiKind::*;

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
                const X: T = &[('\t', '\t'), ('\n', '\n'), ('\x0B', '\x0B'), ('\x0C', '\x0C'), ('\r', '\r'), (' ', ' ')];
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

    let kind = ast::ClassAsciiKind::Word;
    let result = ascii_class(&kind);
    let expected: &[(char, char)] = &[('0', '9'), ('A', 'Z'), ('_', '_'), ('a', 'z')];
    
    assert_eq!(result, expected);
}

