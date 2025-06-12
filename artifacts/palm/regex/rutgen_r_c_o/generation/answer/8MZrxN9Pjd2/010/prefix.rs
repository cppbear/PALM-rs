// Answer 0

#[test]
fn test_visit_pre_word_boundary_unicode_negate() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let hir = Hir::word_boundary(hir::WordBoundary::UnicodeNegate);
    let _ = writer.visit_pre(&hir);
}

#[test]
fn test_visit_pre_word_boundary_ascii() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let hir = Hir::word_boundary(hir::WordBoundary::Ascii);
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
fn test_visit_pre_word_boundary_ascii_negate() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let hir = Hir::word_boundary(hir::WordBoundary::AsciiNegate);
    let _ = writer.visit_pre(&hir);
}

