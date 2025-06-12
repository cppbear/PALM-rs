// Answer 0

#[test]
fn test_hirkind_has_subexprs_empty() {
    let hir_kind = HirKind::Empty;
    hir_kind.has_subexprs();
}

#[test]
fn test_hirkind_has_subexprs_literal() {
    let literal = Literal { span: Span::default(), kind: LiteralKind::Unicode('a'), c: 'a' };
    let hir_kind = HirKind::Literal(literal);
    hir_kind.has_subexprs();
}

#[test]
fn test_hirkind_has_subexprs_unicode_class() {
    let unicode_class = Class::Unicode(ClassUnicode::default());
    let hir_kind = HirKind::Class(unicode_class);
    hir_kind.has_subexprs();
}

#[test]
fn test_hirkind_has_subexprs_bytes_class() {
    let bytes_class = Class::Bytes(ClassBytes::default());
    let hir_kind = HirKind::Class(bytes_class);
    hir_kind.has_subexprs();
}

#[test]
fn test_hirkind_has_subexprs_word_boundary_unicode() {
    let word_boundary = WordBoundary::Unicode;
    let hir_kind = HirKind::WordBoundary(word_boundary);
    hir_kind.has_subexprs();
}

#[test]
fn test_hirkind_has_subexprs_word_boundary_ascii() {
    let word_boundary = WordBoundary::Ascii;
    let hir_kind = HirKind::WordBoundary(word_boundary);
    hir_kind.has_subexprs();
}

#[test]
fn test_hirkind_has_subexprs_anchor_start_line() {
    let anchor = Anchor::StartLine;
    let hir_kind = HirKind::Anchor(anchor);
    hir_kind.has_subexprs();
}

#[test]
fn test_hirkind_has_subexprs_anchor_end_text() {
    let anchor = Anchor::EndText;
    let hir_kind = HirKind::Anchor(anchor);
    hir_kind.has_subexprs();
}

