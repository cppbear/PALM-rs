// Answer 0

#[test]
fn test_visit_post_with_empty() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let hir = Hir::empty();
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_with_literal() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let literal = Literal::new('a');
    let hir = Hir::literal(literal);
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_with_class() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let class = Class::new(vec!['a', 'b', 'c']);
    let hir = Hir::class(class);
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_with_anchor() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let anchor = Anchor::new();
    let hir = Hir::anchor(anchor);
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_with_word_boundary() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let word_boundary = WordBoundary::new();
    let hir = Hir::word_boundary(word_boundary);
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_with_concat() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let concats = vec![Hir::literal(Literal::new('a')), Hir::literal(Literal::new('b'))];
    let hir = Hir::concat(concats);
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_with_alternation() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let alternatives = vec![Hir::literal(Literal::new('a')), Hir::literal(Literal::new('b'))];
    let hir = Hir::alternation(alternatives);
    writer.visit_post(&hir).unwrap();
}

