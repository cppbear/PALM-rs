// Answer 0

#[test]
fn test_visit_pre_word_boundary_ascii() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };
    let hir = Hir::word_boundary(WordBoundary::Ascii);
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_class_bytes() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };
    let class_bytes = ClassBytes::new(vec![ClassBytesRange::new(0x20, 0x7E)]);
    let hir = Hir::class(Class::Bytes(class_bytes));
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_word_boundary_unicode() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };
    let hir = Hir::word_boundary(WordBoundary::Unicode);
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_class_unicode() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };
    let class_unicode = ClassUnicode::new(vec![ClassUnicodeRange::new('A', 'Z')]);
    let hir = Hir::class(Class::Unicode(class_unicode));
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_word_boundary_unicode_negate() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };
    let hir = Hir::word_boundary(WordBoundary::UnicodeNegate);
    writer.visit_pre(&hir).unwrap();
}

