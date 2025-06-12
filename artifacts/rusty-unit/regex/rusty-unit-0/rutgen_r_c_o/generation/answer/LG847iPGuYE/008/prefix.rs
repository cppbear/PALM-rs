// Answer 0

#[test]
fn test_visit_post_repetition_bounded_greedy_false() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let hir = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: hir::RepetitionKind::Range(hir::RepetitionRange::Bounded(1, 5)),
            greedy: false,
            hir: Box::new(Hir::empty()),
        }),
        info: Default::default(),
    };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let _ = writer.visit_post(&hir);
}

#[test]
fn test_visit_post_repetition_bounded_greedy_false_max() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let hir = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: hir::RepetitionKind::Range(hir::RepetitionRange::Bounded(100, 100)),
            greedy: false,
            hir: Box::new(Hir::empty()),
        }),
        info: Default::default(),
    };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let _ = writer.visit_post(&hir);
}

#[test]
fn test_visit_post_repetition_bounded_greedy_false_multiple() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let hir = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: hir::RepetitionKind::Range(hir::RepetitionRange::Bounded(10, 20)),
            greedy: false,
            hir: Box::new(Hir::empty()),
        }),
        info: Default::default(),
    };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let _ = writer.visit_post(&hir);
}

