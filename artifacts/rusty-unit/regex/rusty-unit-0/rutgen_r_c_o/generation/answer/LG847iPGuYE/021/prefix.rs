// Answer 0

#[test]
fn test_visit_post_repetition_zero_or_more() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let hir = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: hir::RepetitionKind::ZeroOrMore,
            greedy: true,
            hir: Box::new(Hir::empty()),
        }),
        info: HirInfo::default(),
    };
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_repetition_zero_or_more_non_greedy() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let hir = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: hir::RepetitionKind::ZeroOrMore,
            greedy: false,
            hir: Box::new(Hir::empty()),
        }),
        info: HirInfo::default(),
    };
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_repetition_one_or_more() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let hir = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: hir::RepetitionKind::OneOrMore,
            greedy: true,
            hir: Box::new(Hir::empty()),
        }),
        info: HirInfo::default(),
    };
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_repetition_exactly() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let hir = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: hir::RepetitionKind::Range(hir::RepetitionRange::Exactly(5)),
            greedy: true,
            hir: Box::new(Hir::empty()),
        }),
        info: HirInfo::default(),
    };
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_repetition_at_least() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let hir = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: hir::RepetitionKind::Range(hir::RepetitionRange::AtLeast(2)),
            greedy: true,
            hir: Box::new(Hir::empty()),
        }),
        info: HirInfo::default(),
    };
    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_repetition_bounded() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let hir = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: hir::RepetitionKind::Range(hir::RepetitionRange::Bounded(2, 5)),
            greedy: true,
            hir: Box::new(Hir::empty()),
        }),
        info: HirInfo::default(),
    };
    writer.visit_post(&hir).unwrap();
}

