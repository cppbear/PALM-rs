// Answer 0

#[test]
fn test_fmt_empty_hir() {
    let hir = Hir {
        kind: HirKind::Empty,
        info: HirInfo { bools: 0 },
    };
    let mut output = String::new();
    let result = write!(&mut output, "{}", hir);
    assert!(result.is_ok());
}

#[test]
fn test_fmt_literal_hir() {
    let hir = Hir {
        kind: HirKind::Literal(Literal::from('a')),
        info: HirInfo { bools: 0 },
    };
    let mut output = String::new();
    let result = write!(&mut output, "{}", hir);
    assert!(result.is_ok());
    assert_eq!(output, "a");
}

#[test]
fn test_fmt_class_hir() {
    let hir = Hir {
        kind: HirKind::Class(Class::new(vec!['a', 'b', 'c'])),
        info: HirInfo { bools: 0 },
    };
    let mut output = String::new();
    let result = write!(&mut output, "{}", hir);
    assert!(result.is_ok());
}

#[test]
fn test_fmt_concat_hir() {
    let hir = Hir {
        kind: HirKind::Concat(vec![
            Hir {
                kind: HirKind::Literal(Literal::from('a')),
                info: HirInfo { bools: 0 },
            },
            Hir {
                kind: HirKind::Literal(Literal::from('b')),
                info: HirInfo { bools: 0 },
            },
        ]),
        info: HirInfo { bools: 0 },
    };
    let mut output = String::new();
    let result = write!(&mut output, "{}", hir);
    assert!(result.is_ok());
    assert_eq!(output, "ab");
}

#[test]
fn test_fmt_alternation_hir() {
    let hir = Hir {
        kind: HirKind::Alternation(vec![
            Hir {
                kind: HirKind::Literal(Literal::from('a')),
                info: HirInfo { bools: 0 },
            },
            Hir {
                kind: HirKind::Literal(Literal::from('b')),
                info: HirInfo { bools: 0 },
            },
        ]),
        info: HirInfo { bools: 0 },
    };
    let mut output = String::new();
    let result = write!(&mut output, "{}", hir);
    assert!(result.is_ok());
}

