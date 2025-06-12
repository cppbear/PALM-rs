// Answer 0

#[test]
fn test_visit_post_repetition_one_or_more_greedy_false() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let repetition = Repetition {
        kind: RepetitionKind::OneOrMore,
        greedy: false,
        hir: Box::new(Hir::empty()),
    };
    let hir = Hir::repetition(repetition);
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_repetition_one_or_more_greedy_false_with_literals() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let repetition = Repetition {
        kind: RepetitionKind::OneOrMore,
        greedy: false,
        hir: Box::new(Hir::literal(Literal::from('a'))),
    };
    let hir = Hir::repetition(repetition);
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_repetition_one_or_more_greedy_false_with_classes() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let repetition = Repetition {
        kind: RepetitionKind::OneOrMore,
        greedy: false,
        hir: Box::new(Hir::class(Class::new(vec!['a', 'b', 'c']))),
    };
    let hir = Hir::repetition(repetition);
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_repetition_one_or_more_greedy_false_with_complex_repetition() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let inner_repetition = Repetition {
        kind: RepetitionKind::OneOrMore,
        greedy: false,
        hir: Box::new(Hir::literal(Literal::from('b'))),
    };
    let outer_repetition = Repetition {
        kind: RepetitionKind::Repetition(inner_repetition),
        greedy: false,
        hir: Box::new(Hir::literal(Literal::from('a'))),
    };
    let hir = Hir::repetition(outer_repetition);
    writer.visit_post(&hir).unwrap();
}

