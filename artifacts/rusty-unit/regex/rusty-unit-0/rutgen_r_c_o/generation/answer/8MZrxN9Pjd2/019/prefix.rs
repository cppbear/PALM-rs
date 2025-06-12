// Answer 0

#[test]
fn test_visit_pre_anchor_start_line() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let hir = Hir { kind: HirKind::Anchor(hir::Anchor::StartLine), info: HirInfo::default() };
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_anchor_end_line() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let hir = Hir { kind: HirKind::Anchor(hir::Anchor::EndLine), info: HirInfo::default() };
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_anchor_start_text() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let hir = Hir { kind: HirKind::Anchor(hir::Anchor::StartText), info: HirInfo::default() };
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_anchor_end_text() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let hir = Hir { kind: HirKind::Anchor(hir::Anchor::EndText), info: HirInfo::default() };
    writer.visit_pre(&hir).unwrap();
}

