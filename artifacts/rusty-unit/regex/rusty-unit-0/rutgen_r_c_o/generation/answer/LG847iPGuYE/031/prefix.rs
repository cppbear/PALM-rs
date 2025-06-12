// Answer 0

#[test]
fn test_visit_post_with_class() {
    let mut wtr = String::new();
    let printer = &mut Printer { _priv: () };
    let writer = Writer { printer, wtr: &mut wtr };
    let class_hir = Hir::class(Class::new(vec!['a', 'b'], false));
    let _ = writer.visit_post(&class_hir);
}

#[test]
fn test_visit_post_with_literal() {
    let mut wtr = String::new();
    let printer = &mut Printer { _priv: () };
    let writer = Writer { printer, wtr: &mut wtr };
    let literal_hir = Hir::literal(Literal::new('a'));
    let _ = writer.visit_post(&literal_hir);
}

#[test]
fn test_visit_post_with_empty() {
    let mut wtr = String::new();
    let printer = &mut Printer { _priv: () };
    let writer = Writer { printer, wtr: &mut wtr };
    let empty_hir = Hir::empty();
    let _ = writer.visit_post(&empty_hir);
}

#[test]
fn test_visit_post_with_anchor() {
    let mut wtr = String::new();
    let printer = &mut Printer { _priv: () };
    let writer = Writer { printer, wtr: &mut wtr };
    let anchor_hir = Hir::anchor(Anchor::new());
    let _ = writer.visit_post(&anchor_hir);
}

#[test]
fn test_visit_post_with_word_boundary() {
    let mut wtr = String::new();
    let printer = &mut Printer { _priv: () };
    let writer = Writer { printer, wtr: &mut wtr };
    let word_boundary_hir = Hir::word_boundary(WordBoundary::new());
    let _ = writer.visit_post(&word_boundary_hir);
}

#[test]
fn test_visit_post_with_concat() {
    let mut wtr = String::new();
    let printer = &mut Printer { _priv: () };
    let writer = Writer { printer, wtr: &mut wtr };
    let concat_hir = Hir::concat(vec![Hir::literal(Literal::new('a')), Hir::literal(Literal::new('b'))]);
    let _ = writer.visit_post(&concat_hir);
}

#[test]
fn test_visit_post_with_alternation() {
    let mut wtr = String::new();
    let printer = &mut Printer { _priv: () };
    let writer = Writer { printer, wtr: &mut wtr };
    let alternation_hir = Hir::alternation(vec![Hir::literal(Literal::new('a')), Hir::literal(Literal::new('b'))]);
    let _ = writer.visit_post(&alternation_hir);
}

