// Answer 0

#[test]
fn test_is_empty_not_empty_variant() {
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

    let non_empty_hir = Hir { kind: HirKind::Other };
    assert_eq!(non_empty_hir.is_empty(), false);
}

#[test]
fn test_is_empty_another_non_empty_variant() {
    struct Hir {
        kind: HirKind,
    }

    enum HirKind {
        Empty,
        Other,
        Another,
    }

    impl Hir {
        pub fn is_empty(&self) -> bool {
            match self.kind {
                HirKind::Empty => true,
                _ => false,
            }
        }
    }

    let another_non_empty_hir = Hir { kind: HirKind::Another };
    assert_eq!(another_non_empty_hir.is_empty(), false);
}

