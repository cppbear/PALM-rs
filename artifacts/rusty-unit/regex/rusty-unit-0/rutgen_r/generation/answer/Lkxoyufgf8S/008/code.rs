// Answer 0

#[test]
fn test_ascii_class_graph() {
    mod ast {
        pub enum ClassAsciiKind {
            Graph,
        }
    }

    let kind = ast::ClassAsciiKind::Graph;
    let result = ascii_class(&kind);
    
    let expected = &[('!', '~')];
    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_alnum() {
    mod ast {
        pub enum ClassAsciiKind {
            Alnum,
        }
    }

    let kind = ast::ClassAsciiKind::Alnum;
    let result = ascii_class(&kind);
    
    let expected = &[('0', '9'), ('A', 'Z'), ('a', 'z')];
    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_alpha() {
    mod ast {
        pub enum ClassAsciiKind {
            Alpha,
        }
    }

    let kind = ast::ClassAsciiKind::Alpha;
    let result = ascii_class(&kind);
    
    let expected = &[('A', 'Z'), ('a', 'z')];
    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_ascii() {
    mod ast {
        pub enum ClassAsciiKind {
            Ascii,
        }
    }

    let kind = ast::ClassAsciiKind::Ascii;
    let result = ascii_class(&kind);
    
    let expected = &[('\x00', '\x7F')];
    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_blank() {
    mod ast {
        pub enum ClassAsciiKind {
            Blank,
        }
    }

    let kind = ast::ClassAsciiKind::Blank;
    let result = ascii_class(&kind);
    
    let expected = &[(' ', '\t')];
    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_cntrl() {
    mod ast {
        pub enum ClassAsciiKind {
            Cntrl,
        }
    }

    let kind = ast::ClassAsciiKind::Cntrl;
    let result = ascii_class(&kind);
    
    let expected = &[('\x00', '\x1F'), ('\x7F', '\x7F')];
    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_digit() {
    mod ast {
        pub enum ClassAsciiKind {
            Digit,
        }
    }

    let kind = ast::ClassAsciiKind::Digit;
    let result = ascii_class(&kind);
    
    let expected = &[('0', '9')];
    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_lower() {
    mod ast {
        pub enum ClassAsciiKind {
            Lower,
        }
    }

    let kind = ast::ClassAsciiKind::Lower;
    let result = ascii_class(&kind);
    
    let expected = &[('a', 'z')];
    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_print() {
    mod ast {
        pub enum ClassAsciiKind {
            Print,
        }
    }

    let kind = ast::ClassAsciiKind::Print;
    let result = ascii_class(&kind);
    
    let expected = &[(' ', '~')];
    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_punct() {
    mod ast {
        pub enum ClassAsciiKind {
            Punct,
        }
    }

    let kind = ast::ClassAsciiKind::Punct;
    let result = ascii_class(&kind);
    
    let expected = &[('!', '/'), (':', '@'), ('[', '`'), ('{', '~')];
    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_space() {
    mod ast {
        pub enum ClassAsciiKind {
            Space,
        }
    }

    let kind = ast::ClassAsciiKind::Space;
    let result = ascii_class(&kind);
    
    let expected = &[
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
fn test_ascii_class_upper() {
    mod ast {
        pub enum ClassAsciiKind {
            Upper,
        }
    }

    let kind = ast::ClassAsciiKind::Upper;
    let result = ascii_class(&kind);
    
    let expected = &[('A', 'Z')];
    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_word() {
    mod ast {
        pub enum ClassAsciiKind {
            Word,
        }
    }

    let kind = ast::ClassAsciiKind::Word;
    let result = ascii_class(&kind);
    
    let expected = &[('0', '9'), ('A', 'Z'), ('_', '_'), ('a', 'z')];
    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_xdigit() {
    mod ast {
        pub enum ClassAsciiKind {
            Xdigit,
        }
    }

    let kind = ast::ClassAsciiKind::Xdigit;
    let result = ascii_class(&kind);
    
    let expected = &[('0', '9'), ('A', 'F'), ('a', 'f')];
    assert_eq!(result, expected);
}

