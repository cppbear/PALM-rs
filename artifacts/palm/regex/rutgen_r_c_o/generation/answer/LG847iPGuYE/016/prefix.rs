// Answer 0

#[test]
fn test_visit_post_repetition_zero() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };
    
    let hir = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: hir::RepetitionKind::Range(Box::new(hir::RepetitionRange::Exactly(0))),
            greedy: false,
            hir: Box::new(Hir::empty()),
        }),
        info: HirInfo {},
    };

    let _ = writer.visit_post(&hir);
}

#[test]
fn test_visit_post_repetition_one() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };

    let hir = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: hir::RepetitionKind::Range(Box::new(hir::RepetitionRange::Exactly(1))),
            greedy: false,
            hir: Box::new(Hir::empty()),
        }),
        info: HirInfo {},
    };

    let _ = writer.visit_post(&hir);
}

#[test]
fn test_visit_post_repetition_five() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };

    let hir = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: hir::RepetitionKind::Range(Box::new(hir::RepetitionRange::Exactly(5))),
            greedy: false,
            hir: Box::new(Hir::empty()),
        }),
        info: HirInfo {},
    };

    let _ = writer.visit_post(&hir);
}

#[test]
fn test_visit_post_repetition_ninety_nine() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };

    let hir = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: hir::RepetitionKind::Range(Box::new(hir::RepetitionRange::Exactly(99))),
            greedy: false,
            hir: Box::new(Hir::empty()),
        }),
        info: HirInfo {},
    };

    let _ = writer.visit_post(&hir);
}

#[test]
fn test_visit_post_repetition_hundred() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };

    let hir = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: hir::RepetitionKind::Range(Box::new(hir::RepetitionRange::Exactly(100))),
            greedy: false,
            hir: Box::new(Hir::empty()),
        }),
        info: HirInfo {},
    };

    let _ = writer.visit_post(&hir);
}

