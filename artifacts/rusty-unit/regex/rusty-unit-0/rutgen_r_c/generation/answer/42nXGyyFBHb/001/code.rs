// Answer 0

#[test]
fn test_into_kind_empty() {
    let hir = Hir::empty();
    let kind = hir.into_kind();
    assert_eq!(kind, HirKind::Empty);
}

#[test]
fn test_into_kind_literal() {
    let literal = Literal('a');
    let hir = Hir::literal(literal);
    let kind = hir.into_kind();
    assert!(matches!(kind, HirKind::Empty));
}

#[test]
fn test_into_kind_class() {
    let class = Class::new(vec!['a', 'b', 'c']);
    let hir = Hir::class(class);
    let kind = hir.into_kind();
    assert!(matches!(kind, HirKind::Empty));
}

#[test]
fn test_into_kind_anchor() {
    let anchor = Anchor;
    let hir = Hir::anchor(anchor);
    let kind = hir.into_kind();
    assert!(matches!(kind, HirKind::Empty));
}

#[test]
fn test_into_kind_word_boundary() {
    let word_boundary = WordBoundary;
    let hir = Hir::word_boundary(word_boundary);
    let kind = hir.into_kind();
    assert!(matches!(kind, HirKind::Empty));
}

#[test]
fn test_into_kind_repetition() {
    let repetition = Repetition::new();
    let hir = Hir::repetition(repetition);
    let kind = hir.into_kind();
    assert!(matches!(kind, HirKind::Empty));
}

#[test]
fn test_into_kind_group() {
    let group = Group::new(vec![]);
    let hir = Hir::group(group);
    let kind = hir.into_kind();
    assert!(matches!(kind, HirKind::Empty));
}

#[test]
fn test_into_kind_concat() {
    let concat = vec![Hir::literal(Literal('a')), Hir::literal(Literal('b'))];
    let hir = Hir::concat(concat);
    let kind = hir.into_kind();
    assert!(matches!(kind, HirKind::Empty));
}

#[test]
fn test_into_kind_alternation() {
    let alternation = vec![Hir::literal(Literal('a')), Hir::literal(Literal('b'))];
    let hir = Hir::alternation(alternation);
    let kind = hir.into_kind();
    assert!(matches!(kind, HirKind::Empty));
}

#[test]
fn test_into_kind_dot() {
    let hir = Hir::dot(true);
    let kind = hir.into_kind();
    assert!(matches!(kind, HirKind::Empty));
}

#[test]
fn test_into_kind_any() {
    let hir = Hir::any(false);
    let kind = hir.into_kind();
    assert!(matches!(kind, HirKind::Empty));
}

