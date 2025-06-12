// Answer 0

#[test]
fn test_prefixes_literal_unicode() {
    struct TestHir {
        kind: HirKind,
    }

    impl TestHir {
        fn kind(&self) -> &HirKind {
            &self.kind
        }
    }
    
    let mut lits = Literals::new();
    let expr = TestHir {
        kind: HirKind::Literal(hir::Literal::Unicode('a')),
    };
    prefixes(&expr, &mut lits);
    assert!(!lits.is_empty());
}

#[test]
fn test_prefixes_literal_byte() {
    struct TestHir {
        kind: HirKind,
    }

    impl TestHir {
        fn kind(&self) -> &HirKind {
            &self.kind
        }
    }

    let mut lits = Literals::new();
    let expr = TestHir {
        kind: HirKind::Literal(hir::Literal::Byte(b'a')),
    };
    prefixes(&expr, &mut lits);
    assert_eq!(lits.len(), 1);
}

#[test]
fn test_prefixes_class_unicode() {
    struct TestHir {
        kind: HirKind,
    }

    impl TestHir {
        fn kind(&self) -> &HirKind {
            &self.kind
        }
    }

    let mut lits = Literals::new();
    let expr = TestHir {
        kind: HirKind::Class(hir::Class::Unicode(vec!['a', 'b'].into_iter().collect())),
    };
    prefixes(&expr, &mut lits);
    assert!(lits.contains_class('a'));
    assert!(lits.contains_class('b'));
}

#[test]
fn test_prefixes_class_bytes() {
    struct TestHir {
        kind: HirKind,
    }

    impl TestHir {
        fn kind(&self) -> &HirKind {
            &self.kind
        }
    }

    let mut lits = Literals::new();
    let expr = TestHir {
        kind: HirKind::Class(hir::Class::Bytes(vec![b'a', b'b'].into_iter().collect())),
    };
    prefixes(&expr, &mut lits);
    assert!(lits.contains_byte_class(b'a'));
    assert!(lits.contains_byte_class(b'b'));
}

#[test]
fn test_prefixes_group() {
    struct TestHir {
        kind: HirKind,
    }

    impl TestHir {
        fn kind(&self) -> &HirKind {
            &self.kind
        }
    }

    let mut lits = Literals::new();
    let group_expr = TestHir {
        kind: HirKind::Group { hir: Box::new(TestHir {
            kind: HirKind::Literal(hir::Literal::Unicode('b')),
        }) },
    };
    prefixes(&group_expr, &mut lits);
    assert!(!lits.is_empty());
}

#[test]
fn test_prefixes_empty_concat() {
    struct TestHir {
        kind: HirKind,
    }

    impl TestHir {
        fn kind(&self) -> &HirKind {
            &self.kind
        }
    }

    let mut lits = Literals::new();
    let expr = TestHir {
        kind: HirKind::Concat(vec![]),
    };
    prefixes(&expr, &mut lits);
    assert!(lits.is_empty());
}

#[test]
fn test_prefixes_one_concat() {
    struct TestHir {
        kind: HirKind,
    }

    impl TestHir {
        fn kind(&self) -> &HirKind {
            &self.kind
        }
    }

    let mut lits = Literals::new();
    let single_expr = TestHir {
        kind: HirKind::Concat(vec![TestHir {
            kind: HirKind::Literal(hir::Literal::Unicode('c')),
        }]),
    };
    prefixes(&single_expr, &mut lits);
    assert_eq!(lits.len(), 1);
}

#[test]
fn test_prefixes_multiple_concat() {
    struct TestHir {
        kind: HirKind,
    }

    impl TestHir {
        fn kind(&self) -> &HirKind {
            &self.kind
        }
    }

    let mut lits = Literals::new();
    let multi_expr = TestHir {
        kind: HirKind::Concat(vec![
            TestHir { kind: HirKind::Literal(hir::Literal::Unicode('d')) },
            TestHir { kind: HirKind::Literal(hir::Literal::Unicode('e')) },
        ]),
    };
    prefixes(&multi_expr, &mut lits);
    assert!(lits.len() >= 1);
}

#[test]
fn test_prefixes_alternation() {
    struct TestHir {
        kind: HirKind,
    }

    impl TestHir {
        fn kind(&self) -> &HirKind {
            &self.kind
        }
    }

    let mut lits = Literals::new();
    let alternation_expr = TestHir {
        kind: HirKind::Alternation(vec![
            TestHir { kind: HirKind::Literal(hir::Literal::Unicode('f')) },
            TestHir { kind: HirKind::Literal(hir::Literal::Unicode('g')) },
        ]),
    };
    prefixes(&alternation_expr, &mut lits);
    assert!(lits.len() >= 1);
}

