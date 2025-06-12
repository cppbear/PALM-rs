// Answer 0

#[test]
fn test_visit_pre_start_line() {
    let mut buffer = String::new();
    let printer = &mut Printer { _priv: () };
    let writer = Writer { printer, wtr: &mut buffer };
    let hir = Hir { kind: HirKind::Anchor(Anchor::StartLine), info: HirInfo::default() };
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_end_line() {
    let mut buffer = String::new();
    let printer = &mut Printer { _priv: () };
    let writer = Writer { printer, wtr: &mut buffer };
    let hir = Hir { kind: HirKind::Anchor(Anchor::EndLine), info: HirInfo::default() };
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_start_text() {
    let mut buffer = String::new();
    let printer = &mut Printer { _priv: () };
    let writer = Writer { printer, wtr: &mut buffer };
    let hir = Hir { kind: HirKind::Anchor(Anchor::StartText), info: HirInfo::default() };
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_end_text() {
    let mut buffer = String::new();
    let printer = &mut Printer { _priv: () };
    let writer = Writer { printer, wtr: &mut buffer };
    let hir = Hir { kind: HirKind::Anchor(Anchor::EndText), info: HirInfo::default() };
    writer.visit_pre(&hir).unwrap();
}

