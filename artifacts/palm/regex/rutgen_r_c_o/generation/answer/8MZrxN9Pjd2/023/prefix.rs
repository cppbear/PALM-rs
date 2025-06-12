// Answer 0

#[test]
fn test_visit_pre_anchor_start_line() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

    let hir = Hir::anchor(hir::Anchor::StartLine);
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_anchor_end_line() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

    let hir = Hir::anchor(hir::Anchor::EndLine);
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_anchor_start_text() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

    let hir = Hir::anchor(hir::Anchor::StartText);
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_anchor_end_text() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

    let hir = Hir::anchor(hir::Anchor::EndText);
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_literal_byte() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

    let hir = Hir::literal(hir::Literal::Byte(0));
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_literal_unicode() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

    let hir = Hir::literal(hir::Literal::Unicode('a'));
    writer.visit_pre(&hir).unwrap();
}

