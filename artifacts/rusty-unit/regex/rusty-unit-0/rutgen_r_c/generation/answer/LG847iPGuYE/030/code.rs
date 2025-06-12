// Answer 0

#[test]
fn test_visit_post_with_empty_hir() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let empty_hir = Hir::empty();
    let result = writer.visit_post(&empty_hir);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_visit_post_with_literal_hir() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let literal_hir = Hir::literal(Literal::new('a'));
    let result = writer.visit_post(&literal_hir);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_visit_post_with_class_hir() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let class_hir = Hir::class(Class::new(vec!['a', 'b', 'c']));
    let result = writer.visit_post(&class_hir);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_visit_post_with_anchor_hir() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let anchor_hir = Hir::anchor(Anchor::new());
    let result = writer.visit_post(&anchor_hir);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_visit_post_with_word_boundary_hir() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let word_boundary_hir = Hir::word_boundary(WordBoundary::new());
    let result = writer.visit_post(&word_boundary_hir);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_visit_post_with_concat_hir() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let concat_hir = Hir::concat(vec![
        Hir::literal(Literal::new('a')),
        Hir::literal(Literal::new('b')),
    ]);
    let result = writer.visit_post(&concat_hir);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_visit_post_with_alternation_hir() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let alternation_hir = Hir::alternation(vec![
        Hir::literal(Literal::new('a')),
        Hir::literal(Literal::new('b')),
    ]);
    let result = writer.visit_post(&alternation_hir);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_visit_post_with_repetition_zero_or_more() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let repetition_hir = Hir::repetition(Repetition {
        kind: RepetitionKind::ZeroOrMore,
        greedy: true,
        hir: Box::new(Hir::literal(Literal::new('a'))),
    });
    let result = writer.visit_post(&repetition_hir);
    assert_eq!(result, Ok(()));
}

