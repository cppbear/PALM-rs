// Answer 0

#[test]
fn test_visit_post_with_word_boundary() {
    let mut wtr = Vec::new();
    let mut printer = Printer { _priv: () };
    let hir = Hir {
        kind: HirKind::WordBoundary(WordBoundary {}),
        info: HirInfo {},
    };
    let mut writer = Writer { printer: &mut printer, wtr };
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_with_anchor() {
    let mut wtr = Vec::new();
    let mut printer = Printer { _priv: () };
    let hir = Hir {
        kind: HirKind::Anchor(Anchor {}),
        info: HirInfo {},
    };
    let mut writer = Writer { printer: &mut printer, wtr };
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_with_alternation() {
    let mut wtr = Vec::new();
    let mut printer = Printer { _priv: () };
    let hir = Hir {
        kind: HirKind::Alternation(vec![Hir::empty()]),
        info: HirInfo {},
    };
    let mut writer = Writer { printer: &mut printer, wtr };
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_with_class() {
    let mut wtr = Vec::new();
    let mut printer = Printer { _priv: () };
    let hir = Hir {
        kind: HirKind::Class(Class::new(vec!['a', 'b', 'c'])),
        info: HirInfo {},
    };
    let mut writer = Writer { printer: &mut printer, wtr };
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_with_literal() {
    let mut wtr = Vec::new();
    let mut printer = Printer { _priv: () };
    let hir = Hir {
        kind: HirKind::Literal(Literal::new('a')),
        info: HirInfo {},
    };
    let mut writer = Writer { printer: &mut printer, wtr };
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_with_concat() {
    let mut wtr = Vec::new();
    let mut printer = Printer { _priv: () };
    let hir = Hir {
        kind: HirKind::Concat(vec![
            Hir::literal(Literal::new('a')),
            Hir::class(Class::new(vec!['b', 'c'])),
        ]),
        info: HirInfo {},
    };
    let mut writer = Writer { printer: &mut printer, wtr };
    writer.visit_post(&hir).unwrap();
}

