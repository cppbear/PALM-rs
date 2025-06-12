// Answer 0

#[test]
fn test_visit_post_repetition_range_at_least() {
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer {
        printer,
        wtr: &mut output,
    };

    let hir = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: hir::RepetitionKind::Range(Box::new(hir::RepetitionRange::AtLeast(0))),
            greedy: true,
            hir: Box::new(Hir::empty()),
        }),
        info: HirInfo::default(),
    };

    writer.visit_post(&hir).unwrap();
    let result = output.clone();

    let hir = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: hir::RepetitionKind::Range(Box::new(hir::RepetitionRange::AtLeast(1))),
            greedy: true,
            hir: Box::new(Hir::empty()),
        }),
        info: HirInfo::default(),
    };

    writer.visit_post(&hir).unwrap();
    let result2 = output.clone();

    let hir = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: hir::RepetitionKind::Range(Box::new(hir::RepetitionRange::AtLeast(100))),
            greedy: false,
            hir: Box::new(Hir::empty()),
        }),
        info: HirInfo::default(),
    };

    writer.visit_post(&hir).unwrap();
    let result3 = output.clone();
}

#[test]
fn test_visit_post_repetition_range_bounded() {
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer {
        printer,
        wtr: &mut output,
    };

    let hir = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: hir::RepetitionKind::Range(Box::new(hir::RepetitionRange::Bounded(0, 100))),
            greedy: false,
            hir: Box::new(Hir::empty()),
        }),
        info: HirInfo::default(),
    };

    writer.visit_post(&hir).unwrap();
    let result = output.clone();
}

#[test]
fn test_visit_post_repetition_range_exceed() {
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer {
        printer,
        wtr: &mut output,
    };

    let hir = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: hir::RepetitionKind::Range(Box::new(hir::RepetitionRange::Bounded(10, 5))),
            greedy: false,
            hir: Box::new(Hir::empty()),
        }),
        info: HirInfo::default(),
    };

    writer.visit_post(&hir).unwrap();
    let result = output.clone();
}

