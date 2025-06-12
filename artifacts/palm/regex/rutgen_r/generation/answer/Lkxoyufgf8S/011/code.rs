// Answer 0

#[test]
fn test_ascii_class_blank() {
    mod ast {
        #[derive(Copy, Clone)]
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
    
    let kind = ast::ClassAsciiKind::Blank;
    let result = ascii_class(&kind);
    let expected: &[(char, char)] = &[(' ', '\t')];
    
    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_space() {
    mod ast {
        #[derive(Copy, Clone)]
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
    
    let kind = ast::ClassAsciiKind::Space;
    let result = ascii_class(&kind);
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

