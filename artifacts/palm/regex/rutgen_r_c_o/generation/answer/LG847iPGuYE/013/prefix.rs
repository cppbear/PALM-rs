// Answer 0

#[test]
fn test_visit_post_zero_exactly() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let hir = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: hir::RepetitionKind::Range(hir::RepetitionRange::Exactly(0)),
            greedy: true,
            hir: Box::new(Hir::empty()),
        }),
        info: HirInfo::default(),
    };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let _ = writer.visit_post(&hir);
}

#[test]
fn test_visit_post_one_exactly() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let hir = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: hir::RepetitionKind::Range(hir::RepetitionRange::Exactly(1)),
            greedy: true,
            hir: Box::new(Hir::empty()),
        }),
        info: HirInfo::default(),
    };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let _ = writer.visit_post(&hir);
}

#[test]
fn test_visit_post_two_exactly() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let hir = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: hir::RepetitionKind::Range(hir::RepetitionRange::Exactly(2)),
            greedy: true,
            hir: Box::new(Hir::empty()),
        }),
        info: HirInfo::default(),
    };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let _ = writer.visit_post(&hir);
}

#[test]
fn test_visit_post_upper_bound_exactly() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let hir = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: hir::RepetitionKind::Range(hir::RepetitionRange::Exactly(u32::MAX)),
            greedy: true,
            hir: Box::new(Hir::empty()),
        }),
        info: HirInfo::default(),
    };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let _ = writer.visit_post(&hir);
}

#[test]
fn test_visit_post_zero_exactly_non_greedy() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let hir = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: hir::RepetitionKind::Range(hir::RepetitionRange::Exactly(0)),
            greedy: false,
            hir: Box::new(Hir::empty()),
        }),
        info: HirInfo::default(),
    };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let _ = writer.visit_post(&hir);
}

