// Answer 0

#[test]
fn test_hirkind_has_subexprs_empty() {
    struct HirKind {
        kind: HirKindVariant,
    }

    enum HirKindVariant {
        Empty,
        Literal(char),
        Class(Vec<char>),
        Anchor(char),
        WordBoundary(char),
        Group(Box<HirKind>),
        Repetition(Box<HirKind>),
        Concat(Vec<HirKind>),
        Alternation(Vec<HirKind>),
    }

    impl HirKind {
        pub fn has_subexprs(&self) -> bool {
            match self.kind {
                HirKindVariant::Empty
                | HirKindVariant::Literal(_) 
                | HirKindVariant::Class(_) 
                | HirKindVariant::Anchor(_) 
                | HirKindVariant::WordBoundary(_) => false,
                HirKindVariant::Group(_) 
                | HirKindVariant::Repetition(_) 
                | HirKindVariant::Concat(_) 
                | HirKindVariant::Alternation(_) => true,
            }
        }
    }

    let empty_hir = HirKind { kind: HirKindVariant::Empty };
    assert!(!empty_hir.has_subexprs());

    let literal_hir = HirKind { kind: HirKindVariant::Literal('a') };
    assert!(!literal_hir.has_subexprs());

    let class_hir = HirKind { kind: HirKindVariant::Class(vec!['a', 'b']) };
    assert!(!class_hir.has_subexprs());

    let anchor_hir = HirKind { kind: HirKindVariant::Anchor('\n') };
    assert!(!anchor_hir.has_subexprs());

    let word_boundary_hir = HirKind { kind: HirKindVariant::WordBoundary('a') };
    assert!(!word_boundary_hir.has_subexprs());
}

