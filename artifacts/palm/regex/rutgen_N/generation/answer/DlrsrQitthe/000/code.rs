// Answer 0

#[test]
fn test_from_bracketed() {
    struct ClassBracketed {
        kind: ClassKind,
    }
    
    struct ClassInduct<'a> {
        kind: &'a ClassKind,
    }
    
    impl ClassInduct<'_> {
        fn from_set(kind: &ClassKind) -> ClassInduct {
            ClassInduct { kind }
        }
    }
    
    enum ClassKind {
        CharSet(Vec<char>),
        // other variants
    }

    let kind = ClassKind::CharSet(vec!['a', 'b', 'c']);
    let ast = ClassBracketed { kind };
    let induct = from_bracketed(&ast);
    
    match induct.kind {
        ClassKind::CharSet(ref chars) => {
            assert_eq!(chars, &vec!['a', 'b', 'c']);
        }
        _ => panic!("Unexpected ClassKind variant"),
    }
}

