// Answer 0

#[test]
fn test_visit_post_zero_or_one_non_greedy() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let repetition = Repetition {
        kind: hir::RepetitionKind::ZeroOrOne,
        greedy: false,
        hir: Box::new(Hir::empty()),
    };
    let hir = Hir {
        kind: HirKind::Repetition(repetition),
        info: HirInfo {},
    };
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_zero_or_more_non_greedy() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let repetition = Repetition {
        kind: hir::RepetitionKind::ZeroOrMore,
        greedy: false,
        hir: Box::new(Hir::empty()),
    };
    let hir = Hir {
        kind: HirKind::Repetition(repetition),
        info: HirInfo {},
    };
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_one_or_more_non_greedy() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let repetition = Repetition {
        kind: hir::RepetitionKind::OneOrMore,
        greedy: false,
        hir: Box::new(Hir::empty()),
    };
    let hir = Hir {
        kind: HirKind::Repetition(repetition),
        info: HirInfo {},
    };
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_at_least_non_greedy() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let repetition = Repetition {
        kind: hir::RepetitionKind::Range(hir::RepetitionRange::AtLeast(3)),
        greedy: false,
        hir: Box::new(Hir::empty()),
    };
    let hir = Hir {
        kind: HirKind::Repetition(repetition),
        info: HirInfo {},
    };
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_bounded_non_greedy() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let repetition = Repetition {
        kind: hir::RepetitionKind::Range(hir::RepetitionRange::Bounded(2, 5)),
        greedy: false,
        hir: Box::new(Hir::empty()),
    };
    let hir = Hir {
        kind: HirKind::Repetition(repetition),
        info: HirInfo {},
    };
    writer.visit_post(&hir).unwrap();
}

