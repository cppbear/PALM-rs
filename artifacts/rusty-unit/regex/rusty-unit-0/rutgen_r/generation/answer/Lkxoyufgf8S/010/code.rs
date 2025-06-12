// Answer 0

#[test]
fn test_ascii_class_cntrl() {
    struct ClassAsciiKind {
        kind: String,
    }

    impl ClassAsciiKind {
        fn new(kind: &str) -> ClassAsciiKind {
            ClassAsciiKind {
                kind: String::from(kind),
            }
        }
    }

    let cntrl_kind = ClassAsciiKind::new("Cntrl");
    
    let result = ascii_class(&cntrl_kind);
    let expected: &[(char, char)] = &[
        ('\x00', '\x1F'),
        ('\x7F', '\x7F'),
    ];
    
    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_graph() {
    struct ClassAsciiKind {
        kind: String,
    }

    impl ClassAsciiKind {
        fn new(kind: &str) -> ClassAsciiKind {
            ClassAsciiKind {
                kind: String::from(kind),
            }
        }
    }

    let graph_kind = ClassAsciiKind::new("Graph");

    let result = ascii_class(&graph_kind);
    let expected: &[(char, char)] = &[('!', '~')];
    
    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_digit() {
    struct ClassAsciiKind {
        kind: String,
    }

    impl ClassAsciiKind {
        fn new(kind: &str) -> ClassAsciiKind {
            ClassAsciiKind {
                kind: String::from(kind),
            }
        }
    }

    let digit_kind = ClassAsciiKind::new("Digit");

    let result = ascii_class(&digit_kind);
    let expected: &[(char, char)] = &[('0', '9')];
    
    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_print() {
    struct ClassAsciiKind {
        kind: String,
    }

    impl ClassAsciiKind {
        fn new(kind: &str) -> ClassAsciiKind {
            ClassAsciiKind {
                kind: String::from(kind),
            }
        }
    }

    let print_kind = ClassAsciiKind::new("Print");

    let result = ascii_class(&print_kind);
    let expected: &[(char, char)] = &[(' ', '~')];
    
    assert_eq!(result, expected);
}

