// Answer 0

#[test]
fn test_is_empty_true() {
    struct Hir {
        kind: HirKind,
    }

    enum HirKind {
        Empty,
        NonEmpty,
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
fn test_is_empty_false() {
    struct Hir {
        kind: HirKind,
    }

    enum HirKind {
        Empty,
        NonEmpty,
    }

    impl Hir {
        pub fn is_empty(&self) -> bool {
            match self.kind {
                HirKind::Empty => true,
                _ => false,
            }
        }
    }

    let non_empty_hir = Hir { kind: HirKind::NonEmpty };
    assert!(!non_empty_hir.is_empty());
}

