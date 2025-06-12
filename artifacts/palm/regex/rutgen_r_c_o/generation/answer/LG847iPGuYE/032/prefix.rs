// Answer 0

#[test]
fn test_visit_post_literal() {
    let mut writer = Printer { _priv: () };
    let mut visitor = Writer { printer: &mut writer, wtr: String::new() };
    let literal_hir = Hir::literal(Literal::from('a'));
    let _ = visitor.visit_post(&literal_hir);
}

#[test]
fn test_visit_post_empty() {
    let mut writer = Printer { _priv: () };
    let mut visitor = Writer { printer: &mut writer, wtr: String::new() };
    let empty_hir = Hir::empty();
    let _ = visitor.visit_post(&empty_hir);
}

#[test]
fn test_visit_post_class() {
    let mut writer = Printer { _priv: () };
    let mut visitor = Writer { printer: &mut writer, wtr: String::new() };
    let class_hir = Hir::class(Class::from(vec!['a', 'b', 'c']));
    let _ = visitor.visit_post(&class_hir);
}

#[test]
fn test_visit_post_anchor() {
    let mut writer = Printer { _priv: () };
    let mut visitor = Writer { printer: &mut writer, wtr: String::new() };
    let anchor_hir = Hir::anchor(Anchor::Start);
    let _ = visitor.visit_post(&anchor_hir);
}

#[test]
fn test_visit_post_word_boundary() {
    let mut writer = Printer { _priv: () };
    let mut visitor = Writer { printer: &mut writer, wtr: String::new() };
    let word_boundary_hir = Hir::word_boundary(WordBoundary::Word);
    let _ = visitor.visit_post(&word_boundary_hir);
}

#[test]
fn test_visit_post_concat() {
    let mut writer = Printer { _priv: () };
    let mut visitor = Writer { printer: &mut writer, wtr: String::new() };
    let concat_hir = Hir::concat(vec![Hir::literal(Literal::from('a')), Hir::literal(Literal::from('b'))]);
    let _ = visitor.visit_post(&concat_hir);
}

#[test]
fn test_visit_post_alternation() {
    let mut writer = Printer { _priv: () };
    let mut visitor = Writer { printer: &mut writer, wtr: String::new() };
    let alternation_hir = Hir::alternation(vec![Hir::literal(Literal::from('a')), Hir::literal(Literal::from('b'))]);
    let _ = visitor.visit_post(&alternation_hir);
}

#[test]
fn test_visit_post_repetition_zero_or_one() {
    let mut writer = Printer { _priv: () };
    let mut visitor = Writer { printer: &mut writer, wtr: String::new() };
    let repetition_hir = Hir::repetition(Repetition { kind: hir::RepetitionKind::ZeroOrOne, greedy: true, hir: Box::new(Hir::empty()) });
    let _ = visitor.visit_post(&repetition_hir);
}

#[test]
fn test_visit_post_repetition_zero_or_more() {
    let mut writer = Printer { _priv: () };
    let mut visitor = Writer { printer: &mut writer, wtr: String::new() };
    let repetition_hir = Hir::repetition(Repetition { kind: hir::RepetitionKind::ZeroOrMore, greedy: false, hir: Box::new(Hir::literal(Literal::from('a'))) });
    let _ = visitor.visit_post(&repetition_hir);
}

#[test]
fn test_visit_post_repetition_one_or_more() {
    let mut writer = Printer { _priv: () };
    let mut visitor = Writer { printer: &mut writer, wtr: String::new() };
    let repetition_hir = Hir::repetition(Repetition { kind: hir::RepetitionKind::OneOrMore, greedy: true, hir: Box::new(Hir::class(Class::from(vec!['a', 'b', 'c']))) });
    let _ = visitor.visit_post(&repetition_hir);
}

#[test]
fn test_visit_post_repetition_bounded() {
    let mut writer = Printer { _priv: () };
    let mut visitor = Writer { printer: &mut writer, wtr: String::new() };
    let repetition_hir = Hir::repetition(Repetition { kind: hir::RepetitionKind::Range(hir::RepetitionRange::Bounded(1, 5)), greedy: false, hir: Box::new(Hir::anchor(Anchor::Start)) });
    let _ = visitor.visit_post(&repetition_hir);
}

