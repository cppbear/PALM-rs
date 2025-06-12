// Answer 0

#[test]
fn test_visit_pre_word_boundary_unicode() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

    let hir = Hir::word_boundary(hir::WordBoundary::Unicode);
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_word_boundary_ascii() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

    let hir = Hir::word_boundary(hir::WordBoundary::Ascii);
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_word_boundary_unicode_negate() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

    let hir = Hir::word_boundary(hir::WordBoundary::UnicodeNegate);
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_word_boundary_ascii_negate() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

    let hir = Hir::word_boundary(hir::WordBoundary::AsciiNegate);
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_empty() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

    let hir = Hir::empty();
    writer.visit_pre(&hir).unwrap();
}

