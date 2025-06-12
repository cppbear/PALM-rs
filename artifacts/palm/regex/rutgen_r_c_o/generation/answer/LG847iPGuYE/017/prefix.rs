// Answer 0

#[test]
fn test_visit_post_one_or_more_greedy() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

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
fn test_visit_post_one_or_more_non_greedy() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

    let hir = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: hir::RepetitionKind::OneOrMore,
            greedy: false,
            hir: Box::new(Hir::empty()),
        }),
        info: HirInfo::default(),
    };

    writer.visit_post(&hir).unwrap();
}

#[test]
fn test_visit_post_zero_or_more_greedy() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

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
fn test_visit_post_zero_or_more_non_greedy() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

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
fn test_visit_post_bounded() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

    let hir = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: hir::RepetitionKind::Range(hir::RepetitionRange::Bounded(1, 1)),
            greedy: true,
            hir: Box::new(Hir::empty()),
        }),
        info: HirInfo::default(),
    };

    writer.visit_post(&hir).unwrap();
}

