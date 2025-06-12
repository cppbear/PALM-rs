// Answer 0

#[test]
fn test_hirkind_has_subexprs_empty() {
    let kind = HirKind::Empty;
    assert_eq!(kind.has_subexprs(), false);
}

#[test]
fn test_hirkind_has_subexprs_literal() {
    let kind = HirKind::Literal(Literal {
        span: Span { start: 0, end: 1 },
        kind: LiteralKind::Unicode('a'),
        c: 'a',
    });
    assert_eq!(kind.has_subexprs(), false);
}

#[test]
fn test_hirkind_has_subexprs_word_boundary() {
    let kind = HirKind::WordBoundary(WordBoundary::Unicode);
    assert_eq!(kind.has_subexprs(), false);
}

#[test]
fn test_hirkind_has_subexprs_class() {
    let kind = HirKind::Class(Class::Unicode(ClassUnicode {
        name: String::from("L"),
    }));
    assert_eq!(kind.has_subexprs(), false);
}

#[test]
fn test_hirkind_has_subexprs_anchor() {
    let kind = HirKind::Anchor(Anchor::StartLine);
    assert_eq!(kind.has_subexprs(), false);
}

