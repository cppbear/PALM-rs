// Answer 0

#[test]
fn test_visit_post_alternation() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let writer = Writer { printer: &mut printer, wtr: &mut output };

    let alt_hir = Hir::alternation(vec![Hir::literal(Literal::from('a')), Hir::literal(Literal::from('b'))]);
    writer.visit_post(&alt_hir);
}

#[test]
fn test_visit_post_empty() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let writer = Writer { printer: &mut printer, wtr: &mut output };

    let empty_hir = Hir::empty();
    writer.visit_post(&empty_hir);
}

#[test]
fn test_visit_post_anchor() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let writer = Writer { printer: &mut printer, wtr: &mut output };

    let anchor_hir = Hir::anchor(Anchor::new());
    writer.visit_post(&anchor_hir);
}

#[test]
fn test_visit_post_class() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let writer = Writer { printer: &mut printer, wtr: &mut output };

    let class_hir = Hir::class(Class::new());
    writer.visit_post(&class_hir);
}

#[test]
fn test_visit_post_literal() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let writer = Writer { printer: &mut printer, wtr: &mut output };

    let literal_hir = Hir::literal(Literal::from('c'));
    writer.visit_post(&literal_hir);
}

#[test]
fn test_visit_post_concatenation() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let writer = Writer { printer: &mut printer, wtr: &mut output };

    let concat_hir = Hir::concat(vec![Hir::literal(Literal::from('d')), Hir::literal(Literal::from('e'))]);
    writer.visit_post(&concat_hir);
}

#[test]
fn test_visit_post_word_boundary() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let writer = Writer { printer: &mut printer, wtr: &mut output };

    let word_boundary_hir = Hir::word_boundary(WordBoundary::new());
    writer.visit_post(&word_boundary_hir);
}

