// Answer 0

#[test]
fn test_kind_with_simple_hir() {
    struct Hir {
        kind: HirKind,
    }

    enum HirKind {
        Literal(String),
        Character(char),
    }

    let hir = Hir {
        kind: HirKind::Literal("test".to_string()),
    };

    if let HirKind::Literal(ref value) = *hir.kind() {
        assert_eq!(value, "test");
    } else {
        panic!("Expected a Literal variant.");
    }
}

#[test]
fn test_kind_with_character_hir() {
    struct Hir {
        kind: HirKind,
    }

    enum HirKind {
        Literal(String),
        Character(char),
    }

    let hir = Hir {
        kind: HirKind::Character('a'),
    };

    if let HirKind::Character(value) = *hir.kind() {
        assert_eq!(value, 'a');
    } else {
        panic!("Expected a Character variant.");
    }
}

#[test]
fn test_kind_with_complex_hir() {
    struct Hir {
        kind: HirKind,
    }

    enum HirKind {
        Literal(String),
        Character(char),
        Concat(Vec<Hir>),
    }

    let hir = Hir {
        kind: HirKind::Concat(vec![
            Hir { kind: HirKind::Character('b') },
            Hir { kind: HirKind::Literal("hello".to_string()) },
        ]),
    };

    if let HirKind::Concat(ref vec) = *hir.kind() {
        assert_eq!(vec.len(), 2);
        if let HirKind::Character(value) = &vec[0].kind {
            assert_eq!(value, 'b');
        } else {
            panic!("Expected a Character variant in Concat.");
        }
        if let HirKind::Literal(ref value) = &vec[1].kind {
            assert_eq!(value, "hello");
        } else {
            panic!("Expected a Literal variant in Concat.");
        }
    } else {
        panic!("Expected a Concat variant.");
    }
}

