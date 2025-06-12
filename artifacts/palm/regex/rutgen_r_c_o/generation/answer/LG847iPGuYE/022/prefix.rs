// Answer 0

#[test]
fn test_visit_post_zero_or_more_greedy() {
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer { printer, wtr: &mut output };
    
    let repetition_hir = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: hir::RepetitionKind::ZeroOrMore,
            greedy: true,
            hir: Box::new(Hir::empty()),
        }),
        info: HirInfo::default(),
    };
    
    writer.visit_post(&repetition_hir).unwrap();
}

#[test]
fn test_visit_post_zero_or_more_non_greedy() {
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer { printer, wtr: &mut output };
    
    let repetition_hir = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: hir::RepetitionKind::ZeroOrMore,
            greedy: false,
            hir: Box::new(Hir::empty()),
        }),
        info: HirInfo::default(),
    };
    
    writer.visit_post(&repetition_hir).unwrap();
}

#[test]
fn test_visit_post_one_or_more_greedy() {
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer { printer, wtr: &mut output };
    
    let repetition_hir = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: hir::RepetitionKind::OneOrMore,
            greedy: true,
            hir: Box::new(Hir::empty()),
        }),
        info: HirInfo::default(),
    };
    
    writer.visit_post(&repetition_hir).unwrap();
}

#[test]
fn test_visit_post_one_or_more_non_greedy() {
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer { printer, wtr: &mut output };
    
    let repetition_hir = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: hir::RepetitionKind::OneOrMore,
            greedy: false,
            hir: Box::new(Hir::empty()),
        }),
        info: HirInfo::default(),
    };
    
    writer.visit_post(&repetition_hir).unwrap();
}

#[test]
fn test_visit_post_range_exactly() {
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer { printer, wtr: &mut output };
    
    let repetition_hir = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: hir::RepetitionKind::Range(hir::RepetitionRange::Exactly(3)),
            greedy: true,
            hir: Box::new(Hir::empty()),
        }),
        info: HirInfo::default(),
    };
    
    writer.visit_post(&repetition_hir).unwrap();
}

#[test]
fn test_visit_post_range_at_least() {
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer { printer, wtr: &mut output };
    
    let repetition_hir = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: hir::RepetitionKind::Range(hir::RepetitionRange::AtLeast(2)),
            greedy: true,
            hir: Box::new(Hir::empty()),
        }),
        info: HirInfo::default(),
    };
    
    writer.visit_post(&repetition_hir).unwrap();
}

#[test]
fn test_visit_post_range_bounded() {
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer { printer, wtr: &mut output };
    
    let repetition_hir = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: hir::RepetitionKind::Range(hir::RepetitionRange::Bounded(2, 5)),
            greedy: true,
            hir: Box::new(Hir::empty()),
        }),
        info: HirInfo::default(),
    };
    
    writer.visit_post(&repetition_hir).unwrap();
}

