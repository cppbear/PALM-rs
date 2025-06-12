// Answer 0

#[test]
fn test_visit_pre_literal_byte() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

    let hir = Hir::literal(hir::Literal::Byte(0)); // minimum value
    let _ = writer.visit_pre(&hir);

    let hir = Hir::literal(hir::Literal::Byte(255)); // maximum value
    let _ = writer.visit_pre(&hir);
}

#[test]
fn test_visit_pre_literal_unicode() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

    let hir = Hir::literal(hir::Literal::Unicode('\u{0000}')); // minimum unicode character
    let _ = writer.visit_pre(&hir);

    let hir = Hir::literal(hir::Literal::Unicode('\u{FFFF}')); // maximum unicode character
    let _ = writer.visit_pre(&hir);
}

#[test]
fn test_visit_pre_empty_hir() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

    let hir = Hir::empty(); // empty HIR
    let _ = writer.visit_pre(&hir);
}

