// Answer 0

#[test]
fn test_hirkind_empty() {
    let kind = HirKind::Empty;
    kind.has_subexprs();
}

#[test]
fn test_hirkind_literal() {
    let kind = HirKind::Literal(Literal { 
        span: Span::default(), 
        kind: LiteralKind::Unicode('a'), 
        c: 'a' 
    });
    kind.has_subexprs();
}

#[test]
fn test_hirkind_class() {
    let kind = HirKind::Class(Class::Unicode(ClassUnicode::default()));
    kind.has_subexprs();
}

#[test]
fn test_hirkind_anchor() {
    let kind = HirKind::Anchor(Anchor::StartText);
    kind.has_subexprs();
}

#[test]
fn test_hirkind_word_boundary() {
    let kind = HirKind::WordBoundary(WordBoundary::Unicode);
    kind.has_subexprs();
}

