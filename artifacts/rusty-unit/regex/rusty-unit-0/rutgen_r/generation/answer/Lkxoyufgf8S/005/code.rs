// Answer 0

#[test]
fn test_ascii_class_punct() {
    mod ast {
        pub enum ClassAsciiKind {
            Punct,
        }
    }

    fn ascii_class(kind: &ast::ClassAsciiKind) -> &'static [(char, char)] {
        use ast::ClassAsciiKind::*;

        type T = &'static [(char, char)];
        match *kind {
            Punct => {
                const X: T = &[('!', '/'), (':', '@'), ('[', '`'), ('{', '~')];
                X
            }
            _ => &[],
        }
    }

    let kind = ast::ClassAsciiKind::Punct;
    let result = ascii_class(&kind);
    let expected = &[('!', '/'), (':', '@'), ('[', '`'), ('{', '~')];

    assert_eq!(result, expected);
}

