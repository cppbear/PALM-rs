// Answer 0

#[test]
fn test_empty_hir_kind() {
    struct Empty;
    enum HirKind {
        Empty,
        Literal(char),
        Class(char),
        Anchor(char),
        WordBoundary(char),
        Group(Vec<HirKind>),
        Repetition(Box<HirKind>),
        Concat(Vec<HirKind>),
        Alternation(Vec<HirKind>),
    }
    
    let empty_hir = HirKind::Empty;
    assert!(!empty_hir.has_subexprs());
}

#[test]
fn test_literal_hir_kind() {
    enum HirKind {
        Empty,
        Literal(char),
        Class(char),
        Anchor(char),
        WordBoundary(char),
        Group(Vec<HirKind>),
        Repetition(Box<HirKind>),
        Concat(Vec<HirKind>),
        Alternation(Vec<HirKind>),
    }
    
    let literal_hir = HirKind::Literal('a');
    assert!(!literal_hir.has_subexprs());
}

#[test]
fn test_class_hir_kind() {
    enum HirKind {
        Empty,
        Literal(char),
        Class(char),
        Anchor(char),
        WordBoundary(char),
        Group(Vec<HirKind>),
        Repetition(Box<HirKind>),
        Concat(Vec<HirKind>),
        Alternation(Vec<HirKind>),
    }

    let class_hir = HirKind::Class('A');
    assert!(!class_hir.has_subexprs());
}

#[test]
fn test_anchor_hir_kind() {
    enum HirKind {
        Empty,
        Literal(char),
        Class(char),
        Anchor(char),
        WordBoundary(char),
        Group(Vec<HirKind>),
        Repetition(Box<HirKind>),
        Concat(Vec<HirKind>),
        Alternation(Vec<HirKind>),
    }

    let anchor_hir = HirKind::Anchor('^');
    assert!(!anchor_hir.has_subexprs());
}

#[test]
fn test_word_boundary_hir_kind() {
    enum HirKind {
        Empty,
        Literal(char),
        Class(char),
        Anchor(char),
        WordBoundary(char),
        Group(Vec<HirKind>),
        Repetition(Box<HirKind>),
        Concat(Vec<HirKind>),
        Alternation(Vec<HirKind>),
    }

    let word_boundary_hir = HirKind::WordBoundary('w');
    assert!(!word_boundary_hir.has_subexprs());
}

#[test]
fn test_group_hir_kind() {
    enum HirKind {
        Empty,
        Literal(char),
        Class(char),
        Anchor(char),
        WordBoundary(char),
        Group(Vec<HirKind>),
        Repetition(Box<HirKind>),
        Concat(Vec<HirKind>),
        Alternation(Vec<HirKind>),
    }

    let group_hir = HirKind::Group(vec![HirKind::Literal('a')]);
    assert!(group_hir.has_subexprs());
}

#[test]
fn test_repetition_hir_kind() {
    enum HirKind {
        Empty,
        Literal(char),
        Class(char),
        Anchor(char),
        WordBoundary(char),
        Group(Vec<HirKind>),
        Repetition(Box<HirKind>),
        Concat(Vec<HirKind>),
        Alternation(Vec<HirKind>),
    }

    let repetition_hir = HirKind::Repetition(Box::new(HirKind::Literal('b')));
    assert!(repetition_hir.has_subexprs());
}

#[test]
fn test_concat_hir_kind() {
    enum HirKind {
        Empty,
        Literal(char),
        Class(char),
        Anchor(char),
        WordBoundary(char),
        Group(Vec<HirKind>),
        Repetition(Box<HirKind>),
        Concat(Vec<HirKind>),
        Alternation(Vec<HirKind>),
    }

    let concat_hir = HirKind::Concat(vec![HirKind::Literal('c'), HirKind::Literal('d')]);
    assert!(concat_hir.has_subexprs());
}

#[test]
fn test_alternation_hir_kind() {
    enum HirKind {
        Empty,
        Literal(char),
        Class(char),
        Anchor(char),
        WordBoundary(char),
        Group(Vec<HirKind>),
        Repetition(Box<HirKind>),
        Concat(Vec<HirKind>),
        Alternation(Vec<HirKind>),
    }

    let alternation_hir = HirKind::Alternation(vec![HirKind::Literal('e'), HirKind::Literal('f')]);
    assert!(alternation_hir.has_subexprs());
}

