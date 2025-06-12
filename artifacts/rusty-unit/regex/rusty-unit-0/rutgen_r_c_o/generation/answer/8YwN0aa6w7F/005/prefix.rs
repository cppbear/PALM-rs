// Answer 0

#[test]
fn test_hir_kind_is_empty() {
    let hir_kind = HirKind::Empty;
    let result = hir_kind.has_subexprs();
}

#[test]
fn test_hir_kind_is_literal() {
    let literal = Literal {
        span: Span::default(),
        kind: LiteralKind::Unicode('a'),
        c: 'a',
    };
    let hir_kind = HirKind::Literal(literal);
    let result = hir_kind.has_subexprs();
}

#[test]
fn test_hir_kind_is_class() {
    let class = Class::Unicode(ClassUnicode::default());
    let hir_kind = HirKind::Class(class);
    let result = hir_kind.has_subexprs();
}

#[test]
fn test_hir_kind_is_anchor() {
    let anchor = Anchor::StartLine;
    let hir_kind = HirKind::Anchor(anchor);
    let result = hir_kind.has_subexprs();
}

#[test]
fn test_hir_kind_is_word_boundary() {
    let word_boundary = WordBoundary::Unicode;
    let hir_kind = HirKind::WordBoundary(word_boundary);
    let result = hir_kind.has_subexprs();
}

