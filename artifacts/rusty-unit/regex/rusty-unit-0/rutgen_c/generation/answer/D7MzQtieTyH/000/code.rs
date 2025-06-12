// Answer 0

#[test]
fn test_union_suffixes_non_empty() {
    use hir::{self, Hir, HirKind, Literal as HirLiteral};

    // Setup a sample Hir expression (Unicode literal)
    let hir_expr = Hir {
        kind: HirKind::Literal(HirLiteral::Unicode('a')),
        info: Default::default(),
    };

    let mut literals = Literals::empty();
    literals.set_limit_size(10);
    literals.set_limit_class(10);

    let result = literals.union_suffixes(&hir_expr);
    assert!(result);
    assert!(!literals.is_empty());
}

#[test]
fn test_union_suffixes_empty() {
    use hir::{self, Hir, HirKind, Literal as HirLiteral};

    // Setup a sample Hir expression (Unicode literal)
    let hir_expr = Hir {
        kind: HirKind::Literal(HirLiteral::Unicode('a')),
        info: Default::default(),
    };

    let mut literals = Literals::empty();
    literals.set_limit_size(5);
    literals.set_limit_class(5);
    literals.union_suffixes(&hir_expr);

    // Ensure second call returns false and does not change state
    let result = literals.union_suffixes(&hir_expr);
    assert!(!result);
    assert!(literals.is_empty());
}

#[test]
fn test_union_suffixes_exceed_limit() {
    use hir::{self, Hir, HirKind, Literal as HirLiteral};

    // Setup a sample Hir expression (Byte literal)
    let hir_expr = Hir {
        kind: HirKind::Literal(HirLiteral::Byte(5)),
        info: Default::default(),
    };

    let mut literals = Literals::empty();
    literals.set_limit_size(1);  // Set limit size to 1 for this test
    literals.set_limit_class(3);
    literals.union_suffixes(&hir_expr);

    // Should return false and not alter the literals
    let result = literals.union_suffixes(&hir_expr);
    assert!(!result);
    assert!(literals.is_empty());
}

