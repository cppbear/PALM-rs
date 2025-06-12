// Answer 0

#[test]
fn test_is_empty_with_empty_variant() {
    struct Hir {
        kind: HirKind,
    }

    enum HirKind {
        Empty,
        Other,
    }

    impl Hir {
        pub fn is_empty(&self) -> bool {
            match self.kind {
                HirKind::Empty => true,
                _ => false,
            }
        }
    }

    let empty_hir = Hir { kind: HirKind::Empty };
    assert!(empty_hir.is_empty());
}

#[test]
fn test_is_not_empty_with_other_variant() {
    struct Hir {
        kind: HirKind,
    }

    enum HirKind {
        Empty,
        Other,
    }

    impl Hir {
        pub fn is_empty(&self) -> bool {
            match self.kind {
                HirKind::Empty => true,
                _ => false,
            }
        }
    }

    let other_hir = Hir { kind: HirKind::Other };
    assert!(!other_hir.is_empty());
}

