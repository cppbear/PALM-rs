// Answer 0

#[test]
fn test_visit_post_repetition_bounded_zero_zero() {
    let mut wtr = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut wtr };
    
    let repetition = Repetition {
        kind: RepetitionKind::Range(RepetitionRange::Bounded(0, 0)),
        greedy: true,
        hir: Box::new(Hir::empty()),
    };
    let hir = Hir { kind: HirKind::Repetition(repetition), info: HirInfo::default() };
    
    let _ = writer.visit_post(&hir);
}

#[test]
fn test_visit_post_repetition_bounded_one_one() {
    let mut wtr = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut wtr };
    
    let repetition = Repetition {
        kind: RepetitionKind::Range(RepetitionRange::Bounded(1, 1)),
        greedy: true,
        hir: Box::new(Hir::empty()),
    };
    let hir = Hir { kind: HirKind::Repetition(repetition), info: HirInfo::default() };
    
    let _ = writer.visit_post(&hir);
}

#[test]
fn test_visit_post_repetition_bounded_two_three() {
    let mut wtr = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut wtr };
    
    let repetition = Repetition {
        kind: RepetitionKind::Range(RepetitionRange::Bounded(2, 3)),
        greedy: true,
        hir: Box::new(Hir::empty()),
    };
    let hir = Hir { kind: HirKind::Repetition(repetition), info: HirInfo::default() };
    
    let _ = writer.visit_post(&hir);
}

#[test]
fn test_visit_post_repetition_bounded_zero_five() {
    let mut wtr = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut wtr };
    
    let repetition = Repetition {
        kind: RepetitionKind::Range(RepetitionRange::Bounded(0, 5)),
        greedy: true,
        hir: Box::new(Hir::empty()),
    };
    let hir = Hir { kind: HirKind::Repetition(repetition), info: HirInfo::default() };
    
    let _ = writer.visit_post(&hir);
}

#[test]
fn test_visit_post_repetition_non_greedy_bounded_one_two() {
    let mut wtr = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut wtr };
    
    let repetition = Repetition {
        kind: RepetitionKind::Range(RepetitionRange::Bounded(1, 2)),
        greedy: false,
        hir: Box::new(Hir::empty()),
    };
    let hir = Hir { kind: HirKind::Repetition(repetition), info: HirInfo::default() };
    
    let _ = writer.visit_post(&hir);
}

