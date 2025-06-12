// Answer 0

#[test]
fn test_suffixes_with_non_empty_concat() {
    use hir::{self, HirKind, Class, Literal, Repetition, Anchor};
    
    let mut literals = Literals {
        lits: vec![],
        limit_size: 10,
        limit_class: 10,
    };

    let lit_a = Hir::literal(Literal::Unicode('a'));
    let lit_b = Hir::literal(Literal::Byte(b'b'));
    let group = Hir::group(hir::Group { hir: Box::new(lit_b), capture: false });

    let concat_expr = Hir::concat(vec![lit_a.clone(), group]);

    suffixes(&concat_expr, &mut literals);

    assert!(!literals.is_empty());
}

#[test]
fn test_suffixes_with_empty_literals() {
    use hir::{self, HirKind, Class, Literal, Repetition, Anchor};

    let mut literals = Literals {
        lits: vec![],
        limit_size: 10,
        limit_class: 10,
    };

    let concat_expr = Hir::concat(vec![]); // Empty concat

    suffixes(&concat_expr, &mut literals);

    assert!(literals.is_empty());
}

#[test]
fn test_suffixes_with_anchor_end_text() {
    use hir::{self, HirKind, Class, Literal, Repetition, Anchor};

    let mut literals = Literals {
        lits: vec![],
        limit_size: 10,
        limit_class: 10,
    };

    let lit_a = Hir::literal(Literal::Unicode('a'));
    let anchor = Hir::anchor(Anchor::EndText);
    let concat_expr = Hir::concat(vec![lit_a, anchor]);

    suffixes(&concat_expr, &mut literals);

    assert!(!literals.is_empty());
}

#[test]
fn test_suffixes_with_unicode_literal() {
    use hir::{self, HirKind, Class, Literal, Repetition, Anchor};

    let mut literals = Literals {
        lits: vec![],
        limit_size: 10,
        limit_class: 10,
    };

    let unicode_char = Hir::literal(Literal::Unicode('Ω')); // Greek Omega
    let concat_expr = Hir::concat(vec![unicode_char]);

    suffixes(&concat_expr, &mut literals);

    assert!(!literals.is_empty());
}

