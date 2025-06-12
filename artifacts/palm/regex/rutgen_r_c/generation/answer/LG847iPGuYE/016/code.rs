// Answer 0

#[test]
fn test_visit_post_empty() {
    let mut output = String::new();
    let writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let hir = Hir {
        kind: HirKind::Empty,
        info: HirInfo::default(),
    };
    let result = writer.visit_post(&hir);
    assert!(result.is_ok());
    assert_eq!(output, "");
}

#[test]
fn test_visit_post_literal() {
    let mut output = String::new();
    let writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let hir = Hir {
        kind: HirKind::Literal(Literal::new('a')),
        info: HirInfo::default(),
    };
    let result = writer.visit_post(&hir);
    assert!(result.is_ok());
    assert_eq!(output, "");
}

#[test]
fn test_visit_post_repetition_range_exactly() {
    let mut output = String::new();
    let writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let repetition = Repetition {
        kind: hir::RepetitionKind::Range(Box::new(hir::RepetitionRange::Exactly(3))),
        greedy: false,
        hir: Box::new(Hir::empty()),
    };
    let hir = Hir {
        kind: HirKind::Repetition(repetition),
        info: HirInfo::default(),
    };
    let result = writer.visit_post(&hir);
    assert!(result.is_ok());
    assert_eq!(output, "{{3}}?");
}

#[test]
fn test_visit_post_repetition_range_at_least() {
    let mut output = String::new();
    let writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let repetition = Repetition {
        kind: hir::RepetitionKind::Range(Box::new(hir::RepetitionRange::AtLeast(2))),
        greedy: false,
        hir: Box::new(Hir::empty()),
    };
    let hir = Hir {
        kind: HirKind::Repetition(repetition),
        info: HirInfo::default(),
    };
    let result = writer.visit_post(&hir);
    assert!(result.is_ok());
    assert_eq!(output, "{{2},}?");
}

#[test]
fn test_visit_post_repetition_range_bounded() {
    let mut output = String::new();
    let writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let repetition = Repetition {
        kind: hir::RepetitionKind::Range(Box::new(hir::RepetitionRange::Bounded(1, 5))),
        greedy: true,
        hir: Box::new(Hir::empty()),
    };
    let hir = Hir {
        kind: HirKind::Repetition(repetition),
        info: HirInfo::default(),
    };
    let result = writer.visit_post(&hir);
    assert!(result.is_ok());
    assert_eq!(output, "{{1},{5}}");
}

