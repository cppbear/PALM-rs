// Answer 0

#[test]
fn test_visit_pre_with_word_boundary_unicode_negate() {
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let writer = Writer { printer, wtr: &mut output };
    let hir = Hir::word_boundary(hir::WordBoundary::UnicodeNegate);
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_with_word_boundary_ascii() {
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let writer = Writer { printer, wtr: &mut output };
    let hir = Hir::word_boundary(hir::WordBoundary::Ascii);
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_with_word_boundary_ascii_negate() {
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let writer = Writer { printer, wtr: &mut output };
    let hir = Hir::word_boundary(hir::WordBoundary::AsciiNegate);
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_with_word_boundary_unicode() {
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let writer = Writer { printer, wtr: &mut output };
    let hir = Hir::word_boundary(hir::WordBoundary::Unicode);
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_with_literal_byte() {
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let writer = Writer { printer, wtr: &mut output };
    for b in 0..=255 {
        let hir = Hir::literal(hir::Literal::Byte(b));
        writer.visit_pre(&hir).unwrap();
    }
}

#[test]
fn test_visit_pre_with_literal_unicode() {
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let writer = Writer { printer, wtr: &mut output };
    for c in '\u{0000}'..='\u{FFFF}' {
        let hir = Hir::literal(hir::Literal::Unicode(c));
        writer.visit_pre(&hir).unwrap();
    }
}

