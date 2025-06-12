// Answer 0

#[test]
fn test_hirkind_empty() {
    let kind = HirKind::Empty;
    let _ = kind.is_empty();
}

#[test]
fn test_hirkind_literal() {
    let literal = Literal { span: Span::default(), kind: LiteralKind::Unicode('a'), c: 'a' };
    let kind = HirKind::Literal(literal);
    let _ = kind.is_empty();
}

#[test]
fn test_hirkind_class() {
    let class = Class::Unicode(ClassUnicode::new(vec!['a', 'b', 'c']));
    let kind = HirKind::Class(class);
    let _ = kind.is_empty();
}

#[test]
fn test_hirkind_anchor_start_line() {
    let anchor = Anchor::StartLine;
    let kind = HirKind::Anchor(anchor);
    let _ = kind.is_empty();
}

#[test]
fn test_hirkind_word_boundary_unicode() {
    let word_boundary = WordBoundary::Unicode;
    let kind = HirKind::WordBoundary(word_boundary);
    let _ = kind.is_empty();
}

