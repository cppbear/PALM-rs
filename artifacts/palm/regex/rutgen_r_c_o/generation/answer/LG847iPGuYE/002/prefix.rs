// Answer 0

#[test]
fn test_visit_post_concat() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let exprs = vec![Hir::literal(Literal), Hir::class(Class)];
    let hir = Hir::concat(exprs);
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_anchor() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let anchor = Anchor;
    let hir = Hir::anchor(anchor);
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_alternation() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let exprs = vec![Hir::literal(Literal), Hir::word_boundary(WordBoundary)];
    let hir = Hir::alternation(exprs);
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_empty() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let hir = Hir::empty();
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_class() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let class = Class;
    let hir = Hir::class(class);
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_literal() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let literal = Literal;
    let hir = Hir::literal(literal);
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_word_boundary() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let word_boundary = WordBoundary;
    let hir = Hir::word_boundary(word_boundary);
    writer.visit_post(&hir).unwrap();
}

