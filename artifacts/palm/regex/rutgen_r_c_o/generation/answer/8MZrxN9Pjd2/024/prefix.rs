// Answer 0

#[test]
fn test_visit_pre_anchor_start_line() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let hir = Hir::anchor(hir::Anchor::StartLine);
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_empty() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let hir = Hir::empty();
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_anchor_end_line() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let hir = Hir::anchor(hir::Anchor::EndLine);
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_anchor_start_text() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let hir = Hir::anchor(hir::Anchor::StartText);
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_anchor_end_text() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let hir = Hir::anchor(hir::Anchor::EndText);
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.visit_pre(&hir).unwrap();
}

