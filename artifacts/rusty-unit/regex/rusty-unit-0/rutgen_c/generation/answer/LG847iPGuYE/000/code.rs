// Answer 0

#[test]
fn test_visit_post_empty() {
    let mut output = String::new();
    {
        let mut writer = Writer {
            printer: &mut Printer { _priv: () },
            wtr: &mut output,
        };
        let hir = Hir::empty();
        writer.visit_post(&hir).unwrap();
    }
    assert_eq!(output, "");
}

#[test]
fn test_visit_post_literal() {
    let mut output = String::new();
    {
        let mut writer = Writer {
            printer: &mut Printer { _priv: () },
            wtr: &mut output,
        };
        let hir = Hir::literal(Literal::from('a'));
        writer.visit_post(&hir).unwrap();
    }
    assert_eq!(output, "");
}

#[test]
fn test_visit_post_repetition_zero_or_one() {
    let mut output = String::new();
    {
        let mut writer = Writer {
            printer: &mut Printer { _priv: () },
            wtr: &mut output,
        };
        let repetition = Repetition {
            kind: RepetitionKind::ZeroOrOne,
            greedy: true,
            hir: Box::new(Hir::empty()),
        };
        let hir = Hir::repetition(repetition);
        writer.visit_post(&hir).unwrap();
    }
    assert_eq!(output, "?");
}

#[test]
fn test_visit_post_repetition_zero_or_more() {
    let mut output = String::new();
    {
        let mut writer = Writer {
            printer: &mut Printer { _priv: () },
            wtr: &mut output,
        };
        let repetition = Repetition {
            kind: RepetitionKind::ZeroOrMore,
            greedy: true,
            hir: Box::new(Hir::empty()),
        };
        let hir = Hir::repetition(repetition);
        writer.visit_post(&hir).unwrap();
    }
    assert_eq!(output, "*");
}

#[test]
fn test_visit_post_repetition_one_or_more() {
    let mut output = String::new();
    {
        let mut writer = Writer {
            printer: &mut Printer { _priv: () },
            wtr: &mut output,
        };
        let repetition = Repetition {
            kind: RepetitionKind::OneOrMore,
            greedy: true,
            hir: Box::new(Hir::empty()),
        };
        let hir = Hir::repetition(repetition);
        writer.visit_post(&hir).unwrap();
    }
    assert_eq!(output, "+");
}

#[test]
fn test_visit_post_repetition_range_exactly() {
    let mut output = String::new();
    {
        let mut writer = Writer {
            printer: &mut Printer { _priv: () },
            wtr: &mut output,
        };
        let repetition = Repetition {
            kind: RepetitionKind::Range(RepetitionRange::Exactly(3)),
            greedy: true,
            hir: Box::new(Hir::empty()),
        };
        let hir = Hir::repetition(repetition);
        writer.visit_post(&hir).unwrap();
    }
    assert_eq!(output, "{{3}}");
}

#[test]
fn test_visit_post_repetition_range_at_least() {
    let mut output = String::new();
    {
        let mut writer = Writer {
            printer: &mut Printer { _priv: () },
            wtr: &mut output,
        };
        let repetition = Repetition {
            kind: RepetitionKind::Range(RepetitionRange::AtLeast(2)),
            greedy: true,
            hir: Box::new(Hir::empty()),
        };
        let hir = Hir::repetition(repetition);
        writer.visit_post(&hir).unwrap();
    }
    assert_eq!(output, "{{2,}}");
}

#[test]
fn test_visit_post_repetition_range_bounded() {
    let mut output = String::new();
    {
        let mut writer = Writer {
            printer: &mut Printer { _priv: () },
            wtr: &mut output,
        };
        let repetition = Repetition {
            kind: RepetitionKind::Range(RepetitionRange::Bounded(1, 3)),
            greedy: true,
            hir: Box::new(Hir::empty()),
        };
        let hir = Hir::repetition(repetition);
        writer.visit_post(&hir).unwrap();
    }
    assert_eq!(output, "{{1,3}}");
}

#[test]
fn test_visit_post_repetition_non_greedy() {
    let mut output = String::new();
    {
        let mut writer = Writer {
            printer: &mut Printer { _priv: () },
            wtr: &mut output,
        };
        let repetition = Repetition {
            kind: RepetitionKind::ZeroOrMore,
            greedy: false,
            hir: Box::new(Hir::empty()),
        };
        let hir = Hir::repetition(repetition);
        writer.visit_post(&hir).unwrap();
    }
    assert_eq!(output, "*?");
}

