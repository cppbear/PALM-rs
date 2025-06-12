// Answer 0

#[test]
fn test_visit_pre_empty() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let hir = Hir::empty();
    let _ = writer.visit_pre(&hir);
}

#[test]
fn test_visit_pre_anchor_start_line() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let hir = Hir::anchor(hir::Anchor::StartLine);
    let _ = writer.visit_pre(&hir);
}

#[test]
fn test_visit_pre_anchor_end_line() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let hir = Hir::anchor(hir::Anchor::EndLine);
    let _ = writer.visit_pre(&hir);
}

#[test]
fn test_visit_pre_anchor_start_text() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let hir = Hir::anchor(hir::Anchor::StartText);
    let _ = writer.visit_pre(&hir);
}

#[test]
fn test_visit_pre_anchor_end_text() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let hir = Hir::anchor(hir::Anchor::EndText);
    let _ = writer.visit_pre(&hir);
}

